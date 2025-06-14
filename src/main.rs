mod args;
mod cells;
mod collision;
mod image;
mod model;
mod sim;

use cells::{calc_grid_coverage, print_grid};
use clap::Parser;
use colored::Colorize;
use image::save_grid_image;
use model::{init_model};
use rand::SeedableRng;
use sim::{FAILSAFE_TIME_LIMIT, simulation_loop};

fn main() {
    // Record timestamp what the simulation started
    let start_time = chrono::Utc::now();
    let args = args::Args::parse();

    let args_string = format!("{args:#?}");
    // Converts all args to a string for logging purposes
    if args.verbosity > 0 {
        println!(
            "{}\n{}",
            "Simulation started at:".color(colored::Color::Green).bold(),
            start_time.to_rfc3339()
        );
        println!(
            "{}\n{}\n",
            "Arguments used for the simulation:".color(colored::Color::Green).bold(),
            args_string
        );
    }

   

    // Setup random generator with a possible seed from user
    let mut rng = if args.random_seed > 0 {
        rand::rngs::StdRng::seed_from_u64(args.random_seed)
    } else {
        let seed = rand::random::<u64>();
        rand::rngs::StdRng::seed_from_u64(seed)
    };

    let mut sim_model = match init_model(&args, &mut rng) {
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
        sim_model.print_simulation_parameters();
        println!("\nStarting simulation ... ");
    }

    // Start the simulation loop
    simulation_loop(&mut sim_model, &mut rng);

    // Use start time to find out how long the simulation took
    let end_time = chrono::Utc::now();
    sim_model.sim_real_time = end_time.signed_duration_since(start_time);

    // Check if the simulation was aborted by our fail-safe
    if sim_model.sim_time_elapsed >= FAILSAFE_TIME_LIMIT {
        eprintln!(
            "{}",
            "Simulation ABORTED after 1'000'000 simulation seconds to prevent infinite loop!\n"
                .color(colored::Color::Red)
                .bold()
        );
    }

    let (current_coverage_count, current_coverage_percent) =
        calc_grid_coverage(&sim_model.coverage_grid, sim_model.parallel);

    sim_model.coverage_percent = current_coverage_percent;
    sim_model.coverage_count = current_coverage_count;

    if args.verbosity > 1 {
        println!("\n");
        // If either the width or height are larger than 100 don't print the grid to the terminal
        if sim_model.grid_width > 100 || sim_model.grid_height > 100 {
            println!(
                "{}\n",
                "Note: Grid is too large for comfortable terminal output."
                    .color(colored::Color::Yellow)
                    .bold()
            );
        } else {
            println!(
                "Grid size: {}x{}",
                sim_model.grid_width, sim_model.grid_height
            );
            print_grid(
                sim_model.grid_width,
                sim_model.grid_height,
                &sim_model.coverage_grid,
            );
            println!();
        }
    }

    if args.json_output {
        sim_model.print_simulation_results_json();
    } else {
        sim_model.print_simulation_results();
    }

    if args.verbosity > 0 {
        println!("\nSaving image...");
    }

    // Save the coverage grid as a PNG image
    if let Err(err) = save_grid_image(&sim_model.coverage_grid, &sim_model) {
        eprintln!(
            "{} {}",
            "Error saving image:".color(colored::Color::Red).bold(),
            err
        );
    } else if args.verbosity > 0 {
        println!("Grid image saved as: '{}'", sim_model.image_file_name);
    }
}
