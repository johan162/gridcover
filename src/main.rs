mod args;
mod cells;
mod collision;
mod image;
mod model;
mod sim;
mod strategy;
mod vector;

use cells::{calc_grid_coverage, print_grid};
use clap::Parser;
use colored::Colorize;
use image::save_grid_image;
use model::init_model;
use rand::SeedableRng;
use sim::{FAILSAFE_TIME_LIMIT, simulation_loop};

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use toml::{self};

/// Writes the program arguments to a TOML formatted file
///
/// # Arguments
///
/// * `args` - The program arguments structure
/// * `file_path` - Path where the TOML file should be saved
///
/// # Returns
///
/// * `Ok(())` if the file was successfully written
/// * `Err(e)` if there was an error during serialization or file writing
pub fn write_args_to_toml<T: Serialize>(
    args: &T,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert the args struct to a TOML string
    let toml_string = toml::to_string_pretty(&args)?;

    // Create the file and write the TOML string to it
    let path = Path::new(file_path);
    let mut file = File::create(path)?;
    file.write_all(toml_string.as_bytes())?;

    Ok(())
}

/// Reads program arguments from a TOML formatted file
///
/// # Arguments
///
/// * `file_path` - Path of the TOML file to read
///
/// # Returns
///
/// * `Ok(T)` containing the parsed arguments structure if successful
/// * `Err(e)` if there was an error during file reading or deserialization
pub fn read_args_from_toml<T>(file_path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de> + Default,
{
    // Open the file
    let path = Path::new(file_path);
    if !path.exists() {
        // Return default if file doesn't exist
        return Ok(T::default());
    }

    let mut file = File::open(path)?;
    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string)?;

    // Parse the TOML string to the args structure
    let args: T = toml::from_str(&toml_string)?;

    Ok(args)
}

fn main() {
    // Record timestamp what the simulation started
    let start_time = chrono::Utc::now();
    let mut args = args::Args::parse();

    if let Some(args_read_file) = args.args_read_file_name.as_ref() {
        match read_args_from_toml::<args::Args>(args_read_file.as_str()) {
            Ok(read_args) => {
                // Any arguments on the command line will override those read from the file
                args = args.merge_with(read_args);
            }
            Err(err) => {
                eprintln!(
                    "{} {}",
                    "Error reading args from TOML:"
                        .color(colored::Color::Red)
                        .bold(),
                    err
                );
            }
        }
    }

     if let Some(args_write_file) = args.args_write_file_name.as_ref() {
        write_args_to_toml(&args, args_write_file.as_str()).unwrap_or_else(|err| {
            eprintln!(
                "{} {}",
                "Error writing args to TOML:"
                    .color(colored::Color::Red)
                    .bold(),
                err
            );
        })
    }

    // Converts all args to a string for logging purposes
    if args.verbosity > 0 {
        println!(
            "{}\n{}",
            "Simulation started at:".color(colored::Color::Green).bold(),
            start_time.to_rfc3339()
        );
    }

    // Setup random generator with a possible seed from user
    let mut rng = if args.random_seed > 0 {
        rand::rngs::StdRng::seed_from_u64(args.random_seed)
    } else {
        let seed = rand::random::<u64>();
        rand::rngs::StdRng::seed_from_u64(seed)
    };

    let mut model = match init_model(&args, &mut rng) {
        Ok(model) => model,
        Err(err) => {
            eprintln!(
                "{} {}",
                "Error initializing simulation model:"
                    .color(colored::Color::Red)
                    .bold(),
                err
            );
            return;
        }
    };

    if args.verbosity > 0 {
        model.print_simulation_parameters();
        println!("\nStarting simulation ... ");
    }

    // Start the simulation loop
    simulation_loop(&mut model, &mut rng);

    // Use start time to find out how long the simulation took
    let end_time = chrono::Utc::now();
    model.sim_real_time = end_time.signed_duration_since(start_time);

    // Check if the simulation was aborted by our fail-safe
    if model.sim_time_elapsed >= FAILSAFE_TIME_LIMIT {
        eprintln!(
            "{}",
            "Simulation ABORTED after 1'000'000 simulation seconds to prevent infinite loop!\n"
                .color(colored::Color::Red)
                .bold()
        );
    }

    let (current_coverage_count, current_coverage_percent) =
        calc_grid_coverage(&model.coverage_grid, model.parallel);

    model.coverage_percent = current_coverage_percent;
    model.coverage_count = current_coverage_count;

    if args.verbosity > 1 {
        println!("\n");
        // If either the width or height are larger than 100 don't print the grid to the terminal
        if model.grid_cells_x > 100 || model.grid_cells_y > 100 {
            println!(
                "{}\n",
                "Note: Grid is too large for comfortable terminal output."
                    .color(colored::Color::Yellow)
                    .bold()
            );
        } else {
            println!(
                "Grid size: {}x{}",
                model.grid_cells_x, model.grid_cells_y
            );
            print_grid(
                model.grid_cells_x,
                model.grid_cells_y,
                &model.coverage_grid,
            );
            println!();
        }
    }

    if args.json_output {
        println!();
        model.print_simulation_results_json();
    } else {
        println!();
        model.print_simulation_results();
    }

    if model.image_file_name.is_some() {
        if let Err(err) = save_grid_image(&model) {
            eprintln!(
                "{} {}",
                "Error saving image:".color(colored::Color::Red).bold(),
                err
            );
        } else if args.verbosity > 0 {
            println!(
                "Grid image saved as: '{}'",
                model.image_file_name.unwrap()
            );
        }
    }
}
