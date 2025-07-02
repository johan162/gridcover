use crate::model::SimModel;
use colored::Colorize;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn try_video_encoding(model: &SimModel) -> Result<(), Box<dyn std::error::Error>> {
    if model.create_animation {
        // Determine the encoder based on the model settings and OS
        let encoder = get_encoder(model)?;

        // Check if ffmpeg is installed
        if Command::new("ffmpeg").arg("-version").output().is_err() {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!(
                    "{}",
                    "Program 'ffmpeg' must be installed. For Mac OSX use 'brew install ffmpeg'"
                        .color(colored::Color::Red)
                        .bold()
                ),
            )))?;
        }

        // Check that the video output file doesn't already exist
        if Path::new(&model.animation_file_name).exists() {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!(
                    "{}",
                    format!(
                        "{}: '{}'",
                        "Video output file already exists", model.animation_file_name
                    )
                    .color(colored::Color::Red)
                    .bold()
                ),
            )))?;
        }

        if !model.quiet {
            println!(
                "{}",
                "Creating video using H.265 encoding from simulation frames ..."
                    .color(colored::Color::Green)
                    .bold()
            );
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
                "-pix_fmt",
                "yuv420p",
                &model.animation_file_name,
            ])
            .output();

        if video_cmd_output.is_err() || !video_cmd_output.as_ref().unwrap().status.success() {
            Err(Box::new(
                    std::io::Error::other(
                        format!("{}","Failed to create video from results. 'ffmpeg' exists, but the correct ffmpeg H.265/HEVC encoder is perhaps not installed?"
                            .color(colored::Color::Red)
                            .bold()),
                    )
                ))?;
        } else {
            if !model.quiet {
                println!(
                    "{} {}",
                    "Video created successfully:"
                        .color(colored::Color::Green)
                        .bold(),
                    model
                        .animation_file_name
                        .color(colored::Color::Green)
                        .bold()
                );
            }
            if model.delete_frames {
                // Delete te entire output directory
                if let Err(e) = fs::remove_dir_all(&model.frames_dir) {
                    Err(Box::new(std::io::Error::other(format!(
                        "{} '{}' : {}",
                        "Failed to delete frames directory"
                            .color(colored::Color::Red)
                            .bold(),
                        model.frames_dir.as_str(),
                        e
                    ))))?;
                } else if !model.quiet {
                    println!(
                        "{}",
                        "Frames directory deleted successfully."
                            .color(colored::Color::Green)
                            .bold()
                    );
                }
            } else if !model.quiet {
                println!(
                    "{}",
                    "Frames are kept in the output directory."
                        .color(colored::Color::Yellow)
                        .bold()
                );
            }
        }
    }
    Ok(())
}

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
                        "Using NVIDIA hardware encoder (hevc_nvenc)."
                            .color(colored::Color::Yellow)
                            .bold()
                    );
                }
                "hevc_nvenc"
            } else if has_amd_gpu() {
                if !model.quiet {
                    println!(
                        "{}",
                        "Using AMD hardware encoder (hevc_amf)."
                            .color(colored::Color::Yellow)
                            .bold()
                    );
                }
                "hevc_amf"
            } else if has_intel_vaapi() {
                if !model.quiet {
                    println!(
                        "{}",
                        "Using Intel hardware encoder (hevc_vaapi)."
                            .color(colored::Color::Yellow)
                            .bold()
                    );
                }
                "hevc_vaapi"
            } else if has_vulkan_support() {
                if !model.quiet {
                    println!(
                        "{}",
                        "Using Vulkan hardware encoder (hevc_vulkan)."
                            .color(colored::Color::Yellow)
                            .bold()
                    );
                }
                "hevc_vulkan"
            } else {
                if !model.quiet {
                    println!(
                        "{}",
                        "No supported hardware encoder found. Falling back to 'libx265' software encoding."
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
