// use num_cpus;
use clap::Parser;
use colored::Colorize;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use rayon::prelude::*;
use std::fs;
use std::process::Command;

// Examples:
// ./target/release/grunner -o "/tmp/grunner" --delete-frames -v -a "\-S 456 \-W 5 \-H 5 \-s 0.01" -m 10000 -s 5
// ./target/release/grunner -o "/tmp/grunner" --delete-output-dir -v -a "\-M assets/mapex01.yaml \-S 4786 \-W 5 \-H 5 \-s 0.01" -m 100000  -s 6
// ./target/release/grunner -o "/tmp/grunner" --delete-output-dir --delete-frames -v -a "\-M assets/mapex01.yaml \-S 4786 \-W 5 \-H 5 \-s 0.01" -m 200000  -s 7

/// Runner for gridcover over a range of coverage values
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Number of CPU cores to use (default: all)
    #[arg(long, short='j', default_value_t = num_cpus::get())]
    cores: usize,

    /// Output directory for results
    #[arg(long, short = 'o', default_value = "runner_results")]
    output_dir: String,

    /// Common arguments to pass to each gridcover run (quoted string, e.g. "--foo bar")
    #[arg(long, short = 'a', default_value = "")]
    common_args: String,

    /// Add boolean flag for video creation
    #[arg(long, short = 'v', default_value_t = false)]
    video: bool,

    /// Use number of steps as stop condition so we can generate frames step by step
    #[arg(long, short = 'm', default_value_t = 0)]
    max_steps: u64,

    /// Real Time Speedup exponential factor for video creation, factor of 2^speedup
    /// (default: 1, i.e. 2x speedup)
    #[arg(long, short = 's', default_value_t = 1)]
    speedup: usize,

    /// Skip frame generation, just do the video creation from existing frames
    #[arg(long, short = 'S', default_value_t = false)]
    skip_simulation: bool,

    /// Use HW encoding support for video creation
    #[arg(long, short = 'H', default_value_t = true)]
    hw_encoding: bool,

    /// Delete all frames after successful video creation
    #[arg(long, default_value_t = false)]
    delete_frames: bool,

    /// Quiet all output
    #[arg(long, short = 'q', default_value_t = false)]
    quiet: bool,

    /// Delete output directory if it exists
    #[arg(long, default_value_t = false)]
    delete_output_dir: bool,
}

fn insert_decimal(s: &str) -> Option<String> {
    if s.len() < 2 || !s.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    let (head, tail) = s.split_at(s.len() - 1);
    Some(format!("{head}.{tail}"))
}

#[allow(clippy::collapsible_if)]
fn main() {
    let args = Args::parse();

    // Delete output directory if it exists
    if args.delete_output_dir {
        if let Ok(dir) = fs::metadata(&args.output_dir) {
            if dir.is_file() {
                eprintln!(
                    "{}",
                    "Output directory name exists as a file, please choose a different name."
                        .color(colored::Color::Red)
                        .bold()
                );
                return;
            }
            if !args.quiet {
                println!(
                    "{}",
                    "Output directory exists, deleting it..."
                        .color(colored::Color::Yellow)
                        .bold()
                );
            }
            fs::remove_dir_all(&args.output_dir).unwrap_or_else(|_| {
                panic!(
                    "{}",
                    "Failed to delete existing output directory"
                        .color(colored::Color::Red)
                        .bold()
                        .to_string()
                )
            });
        }
    }

    // Number of logical CPU cores
    if !args.quiet {
        println!(
            "{}: {}",
            "CPU cores used".color(colored::Color::Green).bold(),
            args.cores
        );
    }

    // Set the number of threads Rayon will use
    rayon::ThreadPoolBuilder::new()
        .num_threads(args.cores)
        .build_global()
        .unwrap_or_else(|_| {
            panic!(
                "{}",
                "Failed to set Rayon thread pool size"
                    .color(colored::Color::Red)
                    .bold()
                    .to_string()
            )
        });

    // Directory to store images/results
    let output_dir = &args.output_dir;
    fs::create_dir_all(output_dir).unwrap_or_else(|_| {
        panic!(
            "{}",
            "Failed to create output directory '{output_dir}'"
                .color(colored::Color::Red)
                .bold()
                .to_string()
        )
    });

    // Range of coverage values (1 to 99)
    let coverage_values: Vec<u32> = (1..=990).collect();

    let steps: Vec<u64> = if args.max_steps > 0 {
        (1..=args.max_steps).collect()
    } else {
        vec![0] // Default to 0 if max_steps is not set
    };

    // Setup progress bar
    if args.max_steps > 0 {
        if !args.quiet {
            println!(
                "{}",
                "Running simulations for steps ..."
                    .color(colored::Color::Green)
                    .bold()
            );
        }
    } else if !args.quiet {
        println!(
            "{}",
            "Running simulations for coverage values ..."
                .color(colored::Color::Green)
                .bold()
        );
    }

    let speedup = 2u32.pow(args.speedup as u32) as usize;
    if speedup < 1 {
        eprintln!(
            "{}",
            "Speedup factor must be at least 1."
                .color(colored::Color::Red)
                .bold()
        );
        return;
    }

    if !args.skip_simulation {
        let pb = if args.max_steps == 0 {
            ProgressBar::new(coverage_values.len() as u64)
        } else {
            ProgressBar::new((steps.len() / speedup) as u64)
        };

        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:50.cyan/blue}] ({percent}%) {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );
        if !args.quiet {
            pb.set_message("Running simulations ...");
        }
        // Parse common_args string into Vec<String>
        let common_args: Vec<String> = if args.common_args.trim().is_empty() {
            vec![]
        } else {
            shell_words::split(&args.common_args).expect("Failed to parse common_args")
        };

        let db_file = format!("{output_dir}/results.db");

        if args.max_steps > 0 {
            // Use rayon for parallelism
            steps
                .par_iter()
                .enumerate()
                .filter(|(i, _)| i % speedup == 0)
                .for_each(|(_, &step)| {
                    let image_file = format!("{output_dir}/frame_{step:08}.png");

                    let mut sim_args = common_args.clone();
                    sim_args.push("-o".to_string());
                    sim_args.push(image_file.clone());
                    sim_args.push("-Q".to_string());
                    sim_args.push(db_file.clone());
                    sim_args.push("-q".to_string());
                    sim_args.push("true".to_string());
                    sim_args.push("-m".to_string());
                    sim_args.push(step.to_string());

                    let status = Command::new("gridcover")
                        .args(&sim_args)
                        .status()
                        .unwrap_or_else(|_| {
                            panic!(
                                "{}",
                                "Failed to start gridcover process"
                                    .color(colored::Color::Red)
                                    .bold()
                                    .to_string()
                            )
                        });

                    if !status.success() {
                        eprintln!(
                            "{}: {}",
                            "gridcover failed for coverage at step"
                                .color(colored::Color::Red)
                                .bold(),
                            step
                        );
                    }
                    if !args.quiet {
                        pb.inc(1);
                    }
                });
        } else {
            // Use rayon for parallelism
            coverage_values.par_iter().for_each(|&coverage| {
                let image_file = format!("{output_dir}/frame_{coverage:04}.png");

                let mut sim_args = common_args.clone();
                sim_args.push("-o".to_string());
                sim_args.push(image_file.clone());
                sim_args.push("-Q".to_string());
                sim_args.push(db_file.clone());
                sim_args.push("-q".to_string());
                sim_args.push("true".to_string());

                sim_args.push("-c".to_string());
                sim_args.push(
                    insert_decimal(&coverage.to_string()).unwrap_or_else(|| coverage.to_string()),
                );

                let status = Command::new("gridcover")
                    .args(&sim_args)
                    .status()
                    .unwrap_or_else(|_| {
                        panic!(
                            "{}",
                            "Failed to start gridcover process"
                                .color(colored::Color::Red)
                                .bold()
                                .to_string()
                        )
                    });

                if !status.success() {
                    eprintln!(
                        "{}: {}%",
                        "'gridcover' failed for coverage"
                            .color(colored::Color::Red)
                            .bold(),
                        coverage
                    );
                }

                if !args.quiet {
                    // Increment progress bar
                    pb.inc(1);
                }
            });
        }

        if !args.quiet {
            // Finish progress bar
            pb.finish_with_message("All simulations complete!");
        }
    }

    if args.video {
        let encoder = if args.hw_encoding {
            "hevc_videotoolbox"
        } else {
            "libx265"
        };
        // Check if ffmpeg is installed
        if Command::new("ffmpeg").arg("-version").output().is_err() {
            eprintln!(
                "{}",
                "Program 'ffmpeg' must be installed. For Mac OSX use 'brew install ffmpeg'"
                    .color(colored::Color::Red)
                    .bold(),
            );
            return;
        }
        if !args.quiet {
            println!(
                "{}",
                "Creating video using H.265 encoding from simulations ..."
                    .color(colored::Color::Green)
                    .bold()
            );
        }
        // Create optional video from results
        let video_file = format!("{output_dir}/sim.mp4");
        let video_cmd_output = Command::new("ffmpeg")
            .args([
                "-framerate",
                "30",
                "-pattern_type",
                "glob",
                "-i",
                &format!("{output_dir}/frame_*.png"),
                "-c:v",
                encoder,
                "-pix_fmt",
                "yuv420p",
                &video_file,
            ])
            .output();
        if video_cmd_output.is_err() || !video_cmd_output.as_ref().unwrap().status.success() {
            eprintln!(
                "{}",
                "Failed to create video from results. 'ffmpeg' exists, but the correct ffmpeg H.265/HEVC encoder is perhaps not installed?"
                    .color(colored::Color::Red)
                    .bold(),
            );
        } else {
            if !args.quiet {
                println!(
                    "{} {}",
                    "Video created successfully:"
                        .color(colored::Color::Green)
                        .bold(),
                    video_file
                );
            }
            if args.delete_frames {
                // Delete all frames_*.png files in the output directory
                let frame_pattern = format!("{output_dir}/frame_*.png");
                let frame_files: Vec<_> = glob::glob(&frame_pattern)
                    .expect("Failed to read glob pattern")
                    .filter_map(Result::ok)
                    .collect();
                let num_frames = frame_files.len();
                for frame_file in frame_files {
                    if let Err(e) = fs::remove_file(&frame_file) {
                        eprintln!(
                            "{}: {}",
                            "Failed to delete frame file"
                                .color(colored::Color::Red)
                                .bold(),
                            e
                        );
                    }
                }

                if !args.quiet {
                    println!(
                        "{} {}",
                        num_frames,
                        "Frames deleted successfully."
                            .color(colored::Color::Green)
                            .bold()
                    );
                }
            } else if !args.quiet {
                println!(
                    "{}",
                    "Frames are kept in the output directory."
                        .color(colored::Color::Yellow)
                        .bold()
                );
            }
        }
    }
}
