use std::fs;
use std::path::Path;
use std::process::Command;
use crate::model::SimModel;
use colored::Colorize;

pub fn try_video_encoding(model: &SimModel) -> Result<(), Box<dyn std::error::Error>> {
    if model.create_animation {
        let encoder = if model.hw_encoding {
            "hevc_videotoolbox"
        } else {
            "libx265"
        };

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
