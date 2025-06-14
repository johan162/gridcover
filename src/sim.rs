use crate::cells::{calc_grid_coverage, mark_covered_cells};
use crate::collision::{check_collision, collision_strategy};
use crate::model::SimModel;
use rand::Rng;
use std::io::Write;

pub const FAILSAFE_TIME_LIMIT: f64 = 1_000_000.0; // 1 million seconds to prevent infinite loops

pub fn simulation_loop(sim_model: &mut SimModel, rng: &mut impl Rng) {
    // Current bounce count and coverage percentage
    let mut current_bounce_count = 0;
    let mut current_coverage_percent = 0.0;
    let mut coverage_count;
    let mut sim_time_elapsed = 0.0;
    let mut sim_time_elapsed_since_last_charge = 0.0;

    // Initialize the circle position binding it to the bounding box
    let mut current_circle_x = sim_model.bb.limit_x(sim_model.start_x);
    let mut current_circle_y = sim_model.bb.limit_y(sim_model.start_y);

    let mut current_dir_x = sim_model.start_dir_x;
    let mut current_dir_y = sim_model.start_dir_y;

    // let mut rng = rand::rng();
    // Run simulation until the first of the stopping conditions is met
    // - either the specified number of bounces is reached
    // - or the specified simulation time is reached
    // - or the specified coverage limit is reached
    // - or simulations steps
    // In addition we have a hard limit of 1_000_000.0 seconds to prevent infinite loops
    // This is a safety measure in case of misconfiguration
    while (sim_model.stop_bounces == 0
        || (sim_model.stop_bounces > 0 && current_bounce_count < sim_model.stop_bounces))
        && (sim_model.stop_time == 0.0
            || (sim_model.stop_time > 0.0 && sim_time_elapsed < sim_model.stop_time))
        && (sim_model.stop_coverage == 0.0
            || sim_model.stop_coverage > 0.0 && current_coverage_percent < sim_model.stop_coverage)
        && (sim_model.stop_simsteps == 0
            || sim_model.stop_simsteps > 0 && sim_model.sim_steps < sim_model.stop_simsteps)
        && (sim_model.stop_distance == 0.0
            || sim_model.stop_distance > 0.0
                && sim_model.distance_covered < sim_model.stop_distance)
        && sim_time_elapsed < FAILSAFE_TIME_LIMIT
    {
        sim_model.sim_steps += 1;

        // Find and mark all grid cells that are fully covered by the circle at the current position
        mark_covered_cells(
            current_circle_x,
            current_circle_y,
            sim_model.radius,
            sim_model.blade_len,
            sim_model.square_size,
            sim_model.grid_width,
            sim_model.grid_height,
            current_bounce_count,
            &mut sim_model.coverage_grid,
            sim_model.track_center,
            sim_model.parallel,
            sim_model.cutter_type,
        );

        // Calculate the next position of the circle based on the current direction and step size
        let next_pos_x = current_circle_x + current_dir_x * sim_model.step_size;
        let next_pos_y = current_circle_y + current_dir_y * sim_model.step_size;

        // Keep track of how far we have moved
        sim_model.distance_covered += sim_model.step_size;

        // Check for collisions with boundaries
        let collision_detected = check_collision(
            next_pos_x,
            next_pos_y,
            &sim_model.bb,
            &mut current_dir_x,
            &mut current_dir_y,
            &mut current_circle_x,
            &mut current_circle_y,
        );

        if !collision_detected && sim_model.perturb_segment {
            // If perturbation is enabled, we can also perturb the direction randomly while moving in a straight line
            // This is done every square distance travelled
            // We use the step size to determine how many simulation steps we need to cover one square distance
            // This is only done if we are not colliding with a boundary

            // How many sim steps to cover the width/height of the cell/square
            let sim_steps_per_cell = (sim_model.square_size / sim_model.step_size).ceil() as u64;

            if sim_model.sim_steps % sim_steps_per_cell == 0
                && rng.random_bool(sim_model.perturb_segment_percent)
            {
                // Perturb the direction randomly +/- PI radians
                let perturb_angle = rng.random_range(-std::f64::consts::PI..=std::f64::consts::PI);
                let angle = (current_dir_y).atan2(current_dir_x) + perturb_angle;
                current_dir_x = angle.cos();
                current_dir_y = angle.sin();
            }
        }

        if collision_detected {
            current_bounce_count += 1;

            (current_dir_x, current_dir_y) = collision_strategy(
                sim_model.perturb,
                current_dir_x,
                current_dir_y,
                &mut current_circle_x,
                &mut current_circle_y,
                rng,
            );

            let rebound_angle = current_dir_y.atan2(current_dir_x);
            if sim_model.verbosity > 2 {
                println!("Collision detected at ({current_circle_x:.2}, {current_circle_y:.2})");
                println!(
                    "  Collision #{current_bounce_count}: New direction ({current_dir_x:.2}, {current_dir_y:.2}) => angle: {:.1} deg",
                    rebound_angle.to_degrees()
                );
            }
        }

        // Move the circle the next simulation step
        current_circle_x += current_dir_x * sim_model.step_size;
        current_circle_y += current_dir_y * sim_model.step_size;

        // Update time in the simulation
        // sim_time_elapsed is in seconds, so we divide the step size by the velocity to get the time for this step
        // This assumes velocity is in units/second
        sim_time_elapsed += sim_model.step_size / sim_model.velocity;
        sim_time_elapsed_since_last_charge += sim_model.step_size / sim_model.velocity;

        // Check if we should consider battery run-time
        if sim_model.battery_run_time > 0.0 {
            // If the battery run time is set, we need to check if we have reached it
            // Battery run time is in minutes and we have a constant power consumption

            if sim_time_elapsed_since_last_charge > sim_model.battery_run_time * 60.0 {
                // If we have reached or exceeded the battery run time, we stop the simulation
                // We add a random time between 3 and 15 minutes to simulate time for the cutter to find its way back to the charging station
                let random_time = rng.random_range(180.0..=900.0); 
                sim_time_elapsed += random_time;
                if sim_model.show_progress && sim_model.verbosity > 1 {
                    println!(
                        "\nBattery run time reached. Time to find charging station: {:.1} minutes",
                        random_time / 60.0
                    );
                }
                sim_time_elapsed_since_last_charge = 0.0;
                sim_time_elapsed += sim_model.battery_charge_time * 60.0; // Add the charging time in seconds
                sim_model.battery_charge_count += 1;
            }
        }

        // Updated coverage only if it is necessary as a stop condition and in addition only every 20 sim steps as it is expensive
        // as it requries a full scan of he entire grid as we haven't implemented a more efficient way to track coverage
        // incrementally yet
        if sim_model.sim_steps % 20 == 0 {
            (coverage_count, current_coverage_percent) =
                calc_grid_coverage(&sim_model.coverage_grid, sim_model.parallel);
            if sim_model.show_progress {
                if sim_model.battery_run_time > 0.0 {
                    print!(
                        "\rCoverage: {current_coverage_percent:>6.2}% ({coverage_count:>7}/{:>7} cells covered) - Bounces: {current_bounce_count:>4} - Sim-Time: {:02}:{:02}:{:02} - Battery capacity left: {:>5.1}%",
                        sim_model.grid_width * sim_model.grid_height,
                        sim_time_elapsed as u64 / 3600,
                        (sim_time_elapsed as u64 % 3600) / 60,
                        sim_time_elapsed as u64 % 60,
                        100.0 - (sim_time_elapsed_since_last_charge/(sim_model.battery_run_time * 60.0)) * 100.0
                    );
                } else {
                    print!(
                        "\rCoverage: {current_coverage_percent:>6.2}% ({coverage_count:>7}/{:>7} cells covered) - Bounces: {current_bounce_count:>4} - Sim-Time: {:02}:{:02}:{:02}",
                        sim_model.grid_width * sim_model.grid_height,
                        sim_time_elapsed as u64 / 3600,
                        (sim_time_elapsed as u64 % 3600) / 60,
                        sim_time_elapsed as u64 % 60,
                    );
                }
                std::io::stdout().flush().unwrap();
            }
        }
    }
    sim_model.bounce_count = current_bounce_count;
    sim_model.sim_time_elapsed = sim_time_elapsed;
}
