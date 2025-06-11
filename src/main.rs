mod args;
mod cells;
mod collision;
mod image;
mod model;
mod sim;

use cells::{calc_grid_coverage, print_grid};
use clap::Parser;
use collision::BoundingBox;
use colored::Colorize;
use image::save_grid_image;
use model::{CoverageInfo, init_model};
use rand::SeedableRng;
use sim::{FAILSAFE_TIME_LIMIT, simulation_loop};

fn main() {
    // Record timestamp what the simulation started
    let start_time = chrono::Utc::now();
    let args = args::Args::parse();

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

    // Setup data structure (matrix) that Tgracks which cells were covered and during
    // which bounce in matrix
    let mut coverage_grid = vec![vec![CoverageInfo::new(); sim_model.width]; sim_model.height];

    // Calculate the bounding box for the simulation area (the edge of the circle is always within the bounding box)
    let bounding_box = BoundingBox::init(
        sim_model.width,
        sim_model.height,
        sim_model.radius,
        sim_model.square_size,
    );

    if args.verbosity > 0 {
        sim_model.print_simulation_parameters();
        println!("\nStarting simulation ... ");
    }

    // Start the simulation loop
    simulation_loop(&mut sim_model, &bounding_box, &mut coverage_grid, &mut rng);

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
        calc_grid_coverage(&coverage_grid, sim_model.parallel);

    sim_model.coverage_percent = current_coverage_percent;
    sim_model.coverage_count = current_coverage_count;

    if args.verbosity > 1 {
        println!("\n");
        // If either the width or height are larger than 100 don't print the grid to the terminal
        if sim_model.width > 100 || sim_model.height > 100 {
            println!(
                "{}\n",
                "Note: Grid is too large for comfortable terminal output."
                    .color(colored::Color::Yellow)
                    .bold()
            );
        } else {
            println!("Grid size: {}x{}", sim_model.width, sim_model.height);
            print_grid(sim_model.width, sim_model.height, &coverage_grid);
            println!();
        }
    }

    sim_model.print_simulation_results();

    if args.verbosity > 0 {
        println!("\nSaving image...");
    }

    // Save the coverage grid as a PNG image
    if let Err(err) = save_grid_image(
        &coverage_grid,
        &sim_model,
    ) {
        eprintln!(
            "{} {}",
            "Error saving image:".color(colored::Color::Red).bold(),
            err
        );
    } else if args.verbosity > 0 {
        println!("Grid image saved as: '{}'", sim_model.image_file_name);
    }
}
