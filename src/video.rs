use crate::model::SimModel;
use chrono::Duration;
use colored::Colorize;
use ffmpeg_next as ffmpeg;
use image::RgbImage;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;


pub fn try_video_encoding(model: &mut SimModel) -> Result<Duration, Box<dyn std::error::Error>> {
    if model.in_memory_frames {
        if model.create_animation {
            if let Some(ref frames) = model.mem_frames {
                create_video_direct(model, frames)?;
            } else {
                return Err("No frames available in memory".into());
            }
            Ok(Duration::zero())  // Or measure time if needed
        } else {
            println!("In-memory frames detected, but animation creation is disabled.");
            Ok(Duration::zero())
        }
    } else {
        try_video_encoding_cli(model)
    }
}

pub fn create_video_direct(
    model: &SimModel,
    frames: &Vec<RgbImage>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize FFmpeg
    ffmpeg::init().unwrap();
    
    // Get output filename - use model.animation_file_name as base and ensure .mp4 extension
    let output_filename = if model.animation_file_name.ends_with(".mp4") {
        model.animation_file_name.clone()
    } else {
        format!("{}.mp4", model.animation_file_name)
    };
    
    if !model.quiet {
        println!(
            "{}",
            format!("Creating video directly from {} frames at {} FPS using HEVC encoding...", 
                frames.len(), model.frame_rate)
                .color(colored::Color::Green)
                .bold()
        );
    }
    
    if frames.is_empty() {
        return Err("No frames provided for video creation".into());
    }
    
    let first_frame = &frames[0];
    let _width = first_frame.width();
    let _height = first_frame.height();
    
    // For now, let's use a simple approach - save frames temporarily and use CLI approach
    // This is a fallback until we can get the direct ffmpeg API working correctly
    
    // Create temporary directory for frames
    let temp_dir = std::env::temp_dir().join(format!("gridcover_frames_{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir)?;
    
    // Save all frames as temporary PNG files
    for (i, frame) in frames.iter().enumerate() {
        let frame_path = temp_dir.join(format!("frame_{:06}.png", i));
        frame.save(&frame_path)?;
    }
    
    // Use ffmpeg CLI to create video from the temporary frames
    let temp_pattern = temp_dir.join("frame_%06d.png");
    let temp_pattern_str = temp_pattern.to_str().ok_or("Invalid path encoding")?;
    
    // Determine the encoder to use (similar to the CLI approach)
    let encoder = if cfg!(target_os = "macos") {
        "hevc_videotoolbox"
    } else {
        "libx265"  // Software fallback
    };
    
    let video_cmd_output = std::process::Command::new("ffmpeg")
        .args([
            "-y", // Overwrite output file
            "-framerate",
            &model.frame_rate.to_string(),
            "-i",
            temp_pattern_str,
            "-c:v",
            encoder,
            "-pix_fmt",
            "yuv420p",
            &output_filename,
        ])
        .output()?;

    // Clean up temporary directory
    std::fs::remove_dir_all(&temp_dir).ok(); // Ignore errors during cleanup
    
    if !video_cmd_output.status.success() {
        // Try software encoding fallback
        let video_cmd_output = std::process::Command::new("ffmpeg")
            .args([
                "-y", // Overwrite output file
                "-framerate",
                &model.frame_rate.to_string(),
                "-i",
                temp_pattern_str,
                "-c:v",
                "libx265",
                "-allow_sw", "1",
                "-pix_fmt",
                "yuv420p",
                &output_filename,
            ])
            .output()?;
            
        if !video_cmd_output.status.success() {
            let stderr = String::from_utf8_lossy(&video_cmd_output.stderr);
            return Err(format!("FFmpeg failed to create video: {}", stderr).into());
        }
    }
    
    if !model.quiet {
        println!(
            "{} {}",
            "Video created successfully:".color(colored::Color::Green).bold(),
            output_filename.color(colored::Color::Cyan).bold()
        );
    }
    
    Ok(())
}



pub fn try_video_encoding_cli(
    model: &mut SimModel,
) -> Result<Duration, Box<dyn std::error::Error>> {
    let mut duration: Duration = Duration::zero();
    if model.create_animation {
        is_ffmpeg_installed()?;
        let encoder = get_encoder(model)?;
        let animation_file_name = get_animation_file_name(model)?;

        try_print_encoding_info(model, encoder, &animation_file_name);

        if !model.quiet {
            println!(
                "{}",
                format!("Encoding video with at {} FPS ...", model.frame_rate)
                    .color(colored::Color::Green)
                    .bold()
            );
            if model.verbosity == 1 {
                println!(
                    "{}",
                    format!("Running: 'ffmpeg -framerate {} -pattern_type glob -i \"{}/frame_*.png\" -c:v {} -pix_fmt yuv420p \"{}\"'", 
                    model.frame_rate, model.frames_dir, encoder, animation_file_name).color(colored::Color::Blue).bold()
                );
            }
            // Empty IO buffer to make sure the output is flushed immediately
            std::io::stdout().flush().unwrap();
        }
        // Determine the time to do ffmpeg encoding.
        let start_time = std::time::Instant::now();

        // Build the ffmpeg command to create the video
        // Example1 (Linux): ffmpeg -framerate 5 -pattern_type glob -i 'frames_dir/*.png' -c:v hevc_nvenc        -pix_fmt yuv420p cutter_sim.mp4
        // Example2 (MacOS): ffmpeg -framerate 5 -pattern_type glob -i 'frames_dir/*.png' -c:v hevc_videotoolbox -pix_fmt yuv420p cutter_sim.mp4
        //
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
                "-pix_fmt",
                "yuv420p",
                &animation_file_name,
            ])
            .output();

        // Time duration of the ffmpeg command
        duration = chrono::Duration::from_std(start_time.elapsed()).unwrap();

        if video_cmd_output.is_err() || !video_cmd_output.as_ref().unwrap().status.success() {
            if model.verbosity > 0 {
                eprintln!(
                    "{} {}",
                    "Error creating video: 'ffmpeg' command failed. Most likely the H.265/HEVC encoder is not installed or does not support the frame size.".color(colored::Color::Red).bold(),
                    "Retrying with SW encoding fallback options.".color(colored::Color::Red).bold()
                );
                std::io::stderr().flush().unwrap();
            }
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
                    "-allow_sw 1", // Allow software encoding if hardware encoding is not available for some reason
                    "-pix_fmt",
                    "yuv420p",
                    &animation_file_name,
                ])
                .output();
            if video_cmd_output.is_err() || !video_cmd_output.as_ref().unwrap().status.success() {
                let cmd = format!(
                    "ffmpeg -framerate {} -pattern_type glob -i '{}/frame_*.png' -c:v {} -allow_sw 1 -pix_fmt yuv420p {}",
                    model.frame_rate, model.frames_dir, encoder, animation_file_name
                );
                Err(Box::new(
                    std::io::Error::other(
                        format!("{}\n{}",
                            "Error creating video: 'ffmpeg' exists, but the correct ffmpeg H.265/HEVC encoder is perhaps not installed?"
                            .color(colored::Color::Red)
                            .bold(),
                            cmd)
                    )
                ))?;
            }
        } else {
            if !model.quiet {
                println!(
                    "{} {}",
                    "Video created successfully:"
                        .color(colored::Color::Green)
                        .bold(),
                    animation_file_name.color(colored::Color::Cyan).bold()
                );
            }
            if model.delete_frames {
                // Delete the entire output directory
                if let Err(e) = fs::remove_dir_all(&model.frames_dir) {
                    Err(Box::new(std::io::Error::other(format!(
                        "{} '{}' : {}",
                        "Failed to delete frames directory"
                            .color(colored::Color::Red)
                            .bold(),
                        model.frames_dir.as_str(),
                        e
                    ))))?;
                } else if !model.quiet && model.verbosity > 1 {
                    println!(
                        "{}",
                        "Frames directory deleted successfully."
                            .color(colored::Color::Green)
                            .bold()
                    );
                }
            } else if !model.quiet && model.verbosity > 1 {
                println!(
                    "{}",
                    "Frames are kept in the output directory."
                        .color(colored::Color::Green)
                        .bold()
                );
            }
        }
    }
    Ok(duration)
}

fn try_print_encoding_info(model: &SimModel, encoder: &'static str, animation_file_name: &String) {
    if !model.quiet && model.verbosity > 1 {
        println!(
            "{}",
            format!("Creating video using H.265 encoding from frames using \"{encoder}\" encoder")
                .color(colored::Color::Green)
                .bold()
        );
        println!(
                "{} {}",
                "Running command:".color(colored::Color::Green).bold(),
                format!(
                    "ffmpeg -framerate {} -pattern_type glob -i '{}/frame_*.png' -c:v {} -allow_sw 1 -pix_fmt yuv420p {}",
                    model.frame_rate,
                    model.frames_dir,
                    encoder,
                    animation_file_name
                )
                .color(colored::Color::Cyan)
                .bold()
            );
        // Empty IO buffer to make sure the output is flushed immediately
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

