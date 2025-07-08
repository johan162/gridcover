mod args;
mod collision;
mod color_theme;
mod db;
mod image;
mod mapfile;
mod model;
mod sim;
mod strategy;
mod vector;
mod video;

use args::{read_args_from_file, write_args_to_file};
use clap::Parser;
use colored::Colorize;
use db::try_store_result_to_db;
use image::try_save_image;
use mapfile::{load_optional_mapfile, try_apply_mapfile_to_model};
use model::{SimModel, init_model};
use rand::Rng;
use rand::SeedableRng;
use sim::{FAILSAFE_TIME_LIMIT, simulation_loop};
use video::try_video_encoding;

fn set_optional_random_start_position(rng: &mut rand::prelude::StdRng, model: &mut SimModel) {
    // Check if we should randomize the start position
    if model.start_x < 0.0 || model.start_y < 0.0 {
        let mut counter = 0;
        loop {
            model.start_x = rng.random_range(model.radius..(model.grid_width - model.radius));
            model.start_y = rng.random_range(model.radius..(model.grid_height - model.radius));
            if !model.grid.as_ref().unwrap().has_obstacle_in_radius(
                model.start_x,
                model.start_y,
                model.radius,
            ) {
                break;
            }
            counter += 1;
            if counter > 10_000 {
                eprintln!(
                    "{}",
                    "Failed to find a valid start position after 10,000 attempts, aborting. Set start position manually."
                        .color(colored::Color::Red)
                        .bold()
                );
                std::process::exit(1);
            }
        }
    }
}

fn main() {
    let start_time = chrono::Utc::now();
    let mut args = args::Args::parse();

    if let Some(args_read_file) = args.args_read_file_name.as_ref() {
        match read_args_from_file::<args::Args>(args_read_file.as_str()) {
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
                std::process::exit(1);
            }
        }
    }

    if let Some(args_write_file) = args.args_write_file_name.as_ref() {
        write_args_to_file(&args, args_write_file.as_str()).unwrap_or_else(|err| {
            eprintln!(
                "{} {}",
                "Error writing args to TOML:"
                    .color(colored::Color::Red)
                    .bold(),
                err
            );
            std::process::exit(1);
        })
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

    load_optional_mapfile(&args, &mut model);

    // Initialize the grid with the calculated number of cells and cell size
    model.grid = Some(model::grid::Grid::new(
        model.grid_cells_x,
        model.grid_cells_y,
        model.cell_size,
    ));

    try_apply_mapfile_to_model(&mut model);

    // We cannot set a random start position until the map has been loaded
    // as we need a start position that is not in an obstacle
    set_optional_random_start_position(&mut rng, &mut model);

    // Start the simulation loop
    simulation_loop(&mut model, &mut rng);

    // Use start time to find out how long the simulation took
    let end_time = chrono::Utc::now();
    model.cpu_time = end_time.signed_duration_since(start_time);

    // Check if the simulation was aborted by our fail-safe
    if model.sim_time_elapsed >= FAILSAFE_TIME_LIMIT {
        eprintln!(
            "{}",
            "Simulation ABORTED after 604,800 simulation seconds to prevent infinite loop!\n"
                .color(colored::Color::Red)
                .bold()
        );
    }

    (model.coverage_count, model.coverage_percent) = model.grid.as_ref().unwrap().get_coverage();
    model.max_visited_number = model.grid.as_ref().unwrap().get_max_visited_number();
    model.min_visited_number = model.grid.as_ref().unwrap().get_min_visited_number();

    if args.show_progress {
        println!();
    }

    try_store_result_to_db(&args, &model);
    try_save_image(&model, None);
    let ffmpeg_encoding_duration = try_video_encoding(&model).unwrap_or_else(|err| {
        eprintln!(
            "{} {}",
            "Error creating video from simulation frames:"
                .color(colored::Color::Red)
                .bold(),
            err
        );
        chrono::Duration::zero()
    });

    if ffmpeg_encoding_duration != chrono::Duration::zero() {
        model.ffmpeg_encoding_duration = Some(ffmpeg_encoding_duration);
    }

    if args.verbosity > 1 {
        if args.json_output {
            model.print_model_as_json();
        } else {
            println!();
            model.print_model_txt();
        }
    }

    if args.verbosity > 0 {
        if args.json_output {
            model.print_simulation_results_as_json();
        } else {
            println!();
            model.print_simulation_results_txt();
        }
    } else if !args.quiet {
        if args.json_output {
            model.print_simulation_results_short_as_json();
        } else {
            println!();
            model.print_simulation_results_short_txt();
        }
    }
}
