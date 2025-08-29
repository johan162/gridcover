use crate::model::{SimModel, try_delete_frames_dir};
use chrono::Duration;
use colored::Colorize;
use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::process::Command;

pub fn try_video_encoding(model: &mut SimModel) -> Result<Duration, Box<dyn std::error::Error>> {
    let mut duration: Duration = Duration::zero();
    if model.create_animation {
        is_ffmpeg_installed()?;
        let encoder = get_encoder(model)?;
        let animation_file_name = get_animation_file_name(model)?;

        try_print_encoding_info(model, encoder, &animation_file_name);

        // Build the ffmpeg command to create the video
        // Example1 (Linux): ffmpeg -framerate 5 -pattern_type glob -i 'frames_dir/*.png' -c:v hevc_nvenc        -pix_fmt yuv420p cutter_sim.mp4
        // Example2 (MacOS): ffmpeg -framerate 5 -pattern_type glob -i 'frames_dir/*.png' -c:v hevc_videotoolbox -pix_fmt yuv420p cutter_sim.mp4
        let start_time = std::time::Instant::now();
        let video_cmd_output = Command::new("ffmpeg")
            .args([
                "-framerate",
                model.frame_rate.to_string().as_str(),
                "-pattern_type",
                "glob",
                "-i",
                &format!("{}/frame_*.png", model.frames_dir),
                "-c:v",
                encoder,
                "-allow_sw",
                "1",
                "-pix_fmt",
                "yuv420p",
                &animation_file_name,
            ])
            .output();
        duration = chrono::Duration::from_std(start_time.elapsed()).unwrap();

        if video_cmd_output.is_err() || !video_cmd_output.as_ref().unwrap().status.success() {
            if model.verbosity > 0 {
                eprintln!(
                    "{} {}",
                    "Error creating video: 'ffmpeg' command failed. Most likely the H.265/HEVC encoder is not installed or does not support the frame size.\n".color(colored::Color::Yellow).bold(),
                    "Retrying with default encoding fallback options.".color(colored::Color::Yellow).bold()
                );
                std::io::stderr().flush().unwrap();
            }

            if model.verbosity > 0 {
                eprintln!(
                    "{}",
                    "Retrying: 'ffmpeg -framerate 5 -pattern_type glob -i '_frames_dir/*.png' -pix_fmt yuv420p cutter_sim.mp' with default encoding options without specifying the encoder."
                        .color(colored::Color::Blue)
                        .bold()
                );
                std::io::stderr().flush().unwrap();
            }

            let start_time = std::time::Instant::now();
            let video_cmd_output = Command::new("ffmpeg")
                .args([
                    "-framerate",
                    model.frame_rate.to_string().as_str(),
                    "-pattern_type",
                    "glob",
                    "-i",
                    &format!("{}/frame_*.png", model.frames_dir),
                    "-pix_fmt",
                    "yuv420p",
                    &animation_file_name,
                ])
                .output();
            duration = chrono::Duration::from_std(start_time.elapsed()).unwrap();

            if video_cmd_output.is_err() || !video_cmd_output.as_ref().unwrap().status.success() {
                Err(Box::new(
                    std::io::Error::other(
                        format!("{}",
                            "Error creating video: 'ffmpeg' exists, but the correct ffmpeg H.265/HEVC encoder is perhaps not installed?"
                            .color(colored::Color::Red)
                            .bold())
                    )
                ))?;
            }
        }
        if !model.quiet {
            println!(
                "{} {}",
                "Video created successfully: "
                    .color(colored::Color::Green)
                    .bold(),
                animation_file_name.color(colored::Color::Cyan).bold()
            );
        }
        try_delete_frames_dir(model)?;
    }
    Ok(duration)
}

fn try_print_encoding_info(model: &SimModel, encoder: &'static str, animation_file_name: &String) {
    if !model.quiet && model.verbosity > 0 {
        println!(
            "{}",
            format!(
                "Creating video using H.265 with \"{encoder}\" encoder with {frame_rate} FPS",
                frame_rate = model.frame_rate
            )
            .color(colored::Color::Green)
            .bold()
        );
        if model.verbosity > 1 {
            println!(
            "{}",
            format!("ffmpeg -framerate {} -pattern_type glob -i \"{}/frame_*.png\" -c:v {} -allow_sw 1 -pix_fmt yuv420p \"{}\"", 
                model.frame_rate, model.frames_dir, encoder, animation_file_name).color(colored::Color::Blue).bold()
            );
        }
        std::io::stdout().flush().unwrap();
    }
}

fn get_animation_file_name(model: &mut SimModel) -> Result<String, Box<dyn Error>> {
    let mut animation_file_name = model.animation_file_name.clone();
    if Path::new(&animation_file_name).exists() {
        // Try adding numbers 1-9 at the end of the file name
        let mut found_new_name = false;
        for i in 1..=9 {
            let new_file_name =
                format!("{}_{}.mp4", animation_file_name.trim_end_matches(".mp4"), i);
            if !Path::new(&new_file_name).exists() {
                animation_file_name = new_file_name;
                found_new_name = true;
                break;
            }
        }

        if found_new_name {
            if !model.quiet {
                println!(
                    "{}",
                    format!(
                        "{}: '{}'",
                        "Video output file already exists, using a new name", animation_file_name
                    )
                    .color(colored::Color::Yellow)
                    .bold()
                );
            }
            model.animation_file_name = animation_file_name.clone();
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!(
                    "{}",
                    format!(
                        "{}: '{}'",
                        "Video output file already exists", animation_file_name
                    )
                    .color(colored::Color::Red)
                    .bold()
                ),
            )))?;
        }
    }
    Ok(animation_file_name)
}

pub fn is_ffmpeg_installed() -> Result<(), Box<dyn Error>> {
    if Command::new("ffmpeg").arg("-version").output().is_err() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!(
                "{}",
                "Program 'ffmpeg' must be installed. For Mac OSX use 'brew install ffmpeg'"
                    .color(colored::Color::Red)
                    .bold()
            ),
        )));
    }
    Ok(())
}

// The following four detection functions uses heuristics to determine the presence of hardware encoders.
// They are not foolproof, sorry.

fn has_nvidia_gpu() -> bool {
    Command::new("nvidia-smi")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn has_intel_vaapi() -> bool {
    std::path::Path::new("/dev/dri").exists()
        && Command::new("vainfo")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
}

fn has_amd_gpu() -> bool {
    Command::new("lspci")
        .output()
        .map(|output| {
            String::from_utf8_lossy(&output.stdout)
                .to_lowercase()
                .contains("amd")
                || String::from_utf8_lossy(&output.stdout)
                    .to_lowercase()
                    .contains("radeon")
        })
        .unwrap_or(false)
}

fn has_vulkan_support() -> bool {
    Command::new("vulkaninfo")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn get_encoder(model: &SimModel) -> Result<&'static str, Box<dyn Error>> {
    let is_osx = std::env::consts::OS == "macos";
    let is_linux = std::env::consts::OS == "linux";
    let encoder = if is_osx {
        if model.hw_encoding {
            "hevc_videotoolbox"
        } else {
            "libx265"
        }
    } else if is_linux {
        if model.hw_encoding {
            // Priority order: NVIDIA > AMD > Intel > Vulkan > Software
            if has_nvidia_gpu() {
                if !model.quiet {
                    println!(
                        "{}",
                        "Linux: Using NVIDIA hardware encoder (hevc_nvenc)."
                            .color(colored::Color::Green)
                            .bold()
                    );
                }
                "hevc_nvenc"
            } else if has_amd_gpu() {
                if !model.quiet {
                    println!(
                        "{}",
                        "Linux: Using AMD hardware encoder (hevc_amf)."
                            .color(colored::Color::Green)
                            .bold()
                    );
                }
                "hevc_amf"
            } else if has_intel_vaapi() {
                if !model.quiet {
                    println!(
                        "{}",
                        "Linux: Using Intel hardware encoder (hevc_vaapi)."
                            .color(colored::Color::Green)
                            .bold()
                    );
                }
                "hevc_vaapi"
            } else if has_vulkan_support() {
                if !model.quiet {
                    println!(
                        "{}",
                        "Linux: Using Vulkan hardware encoder (hevc_vulkan)."
                            .color(colored::Color::Green)
                            .bold()
                    );
                }
                "hevc_vulkan"
            } else {
                if !model.quiet {
                    println!(
                        "{}",
                        "Linux: No supported hardware encoder found. Falling back to 'libx265' software encoding."
                            .color(colored::Color::Yellow)
                            .bold()
                    );
                }
                "libx265"
            }
        } else {
            "libx265"
        }
    } else {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            format!(
                "{}",
                "Video encoding is only supported on Mac OSX and Linux."
                    .color(colored::Color::Red)
                    .bold()
            ),
        )))?;
    };
    Ok(encoder)
}
