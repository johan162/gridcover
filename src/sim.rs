// use crate::cells::{calc_grid_coverage, mark_covered_cells};
use crate::collision::is_grid_edge;
use crate::model::SimModel;
use crate::strategy::cutter_strategy;
use crate::vector::Vector;
use rand::Rng;
use std::io::Write;

pub const FAILSAFE_TIME_LIMIT: f64 = 7.0 * 24.0 * 3600.0; // 7 days in simulated time to prevent infinite loop

pub fn simulation_loop(model: &mut SimModel, rng: &mut impl Rng) {
    // Current bounce count and coverage percentage
    // let mut current_bounce_count = 0;
    let mut current_coverage_percent = 0.0;
    let mut coverage_cell_count;
    let mut sim_time_elapsed_since_last_charge = 0.0;

    // Initialize the circle position binding it to the bounding box
    let mut cutter_center = Vector {
        x: model.bb.limit_x(model.start_x),
        y: model.bb.limit_y(model.start_y),
    };

    let mut current_dir = Vector {
        x: model.start_dir_x,
        y: model.start_dir_y,
    };

    let one_percent_cells = (model.grid_cells_x * model.grid_cells_y) / 100;
    let steps_per_cell = (model.cell_size / model.step_size).floor() as usize;
    let steps_per_tenth_percent = (one_percent_cells * steps_per_cell / 10).min(50) as u64;

    // Run simulation until the first of the stopping conditions is met
    // - either the specified number of bounces is reached
    // - or the specified simulation time is reached
    // - or the specified coverage limit is reached
    // - or simulations steps
    // In addition we have a hard limit of 1_000_000.0 seconds to prevent infinite loops
    // This is a safety measure in case of misconfiguration
    while (model.stop_bounces == 0
        || (model.stop_bounces > 0 && model.segment_number < model.stop_bounces))
        && (model.stop_time == 0.0
            || (model.stop_time > 0.0 && model.sim_time_elapsed < model.stop_time))
        && (model.stop_coverage == 0.0
            || model.stop_coverage > 0.0 && current_coverage_percent < model.stop_coverage)
        && (model.stop_simsteps == 0
            || model.stop_simsteps > 0 && model.sim_steps < model.stop_simsteps)
        && (model.stop_distance == 0.0
            || model.stop_distance > 0.0 && model.distance_covered < model.stop_distance)
        && model.sim_time_elapsed < FAILSAFE_TIME_LIMIT
    {
        model.sim_steps += 1;

        // Keep track of how far we have moved
        model.distance_covered += model.step_size;

        // Calculate the next position of the circle center based on the current direction and step size
        cutter_center += current_dir * model.step_size;

        // Find and mark all grid cells that are fully covered by the circle at the current position
        model.grid.as_mut().unwrap().mark_covered_cells(
            &cutter_center,
            model.radius,
            model.segment_number,
            model.blade_len,
            model.cutter_type,
            model.track_center,
        );

        // Check for collisions with boundaries
        let mut collision_detected = is_grid_edge(&cutter_center, &model.bb, &mut current_dir);

        // Check if we are colliding with an obstacle
        if !collision_detected
            && model.grid.as_ref().unwrap().has_obstacle_in_radius(
                cutter_center.x,
                cutter_center.y,
                model.radius,
            )
        {
            current_dir.x = -current_dir.x; // Reverse x direction
            current_dir.y = -current_dir.y; // Reverse y direction
            collision_detected = true; // Mark as collision detected 

            // Make the position un-collided by moving one step in reverse direction
            // This is to ensure we don't get stuck in the obstacle
            cutter_center += current_dir * model.step_size;
        }

        if collision_detected {
            model.segment_number += 1;
            // Get the position un-collided
            cutter_center = Vector {
                x: model.bb.limit_x(cutter_center.x),
                y: model.bb.limit_y(cutter_center.y),
            };
        }

        (current_dir.x, current_dir.y) =
            cutter_strategy(&current_dir, &cutter_center, collision_detected, model, rng);

        // Update time in the simulation
        // sim_time_elapsed is in seconds, so we divide the step size by the velocity to get the time for this step
        // This assumes velocity is in units/second
        model.sim_time_elapsed += model.step_size / model.velocity;
        sim_time_elapsed_since_last_charge += model.step_size / model.velocity;

        // Check if we should consider battery run-time
        if model.battery_run_time > 0.0 {
            // If the battery run time is set, we need to check if we have reached it
            // Battery run time is in minutes and we have a constant power consumption

            model.battery_charge_left = 100.0
                - (sim_time_elapsed_since_last_charge / (model.battery_run_time * 60.0)) * 100.0;

            if sim_time_elapsed_since_last_charge > model.battery_run_time * 60.0 {
                // If we have reached or exceeded the battery run time, we stop the simulation
                // We add a random time between 3 and 15 minutes to simulate time for the cutter to find its way back to the charging station
                let random_time = rng.random_range(180.0..=900.0);
                model.sim_time_elapsed += random_time;
                if model.show_progress && model.verbosity > 1 {
                    println!(
                        "\nBattery run time reached. Time to find charging station: {:.1} minutes",
                        random_time / 60.0
                    );
                }
                sim_time_elapsed_since_last_charge = 0.0;
                model.sim_time_elapsed += model.battery_charge_time * 60.0; // Add the charging time in seconds
                model.battery_charge_count += 1;
            }
        }

        if model.sim_steps == 1 || model.sim_steps % steps_per_tenth_percent == 0 {
            (coverage_cell_count, current_coverage_percent) =
                model.grid.as_ref().unwrap().get_coverage();

            if model.show_progress {
                if model.battery_run_time > 0.0 {
                    print!(
                        "\rCoverage: {:>6.2}% ({:>7}/{:>7} cells covered), Distance: {:>6.2}, Bounces: {:>4}, Sim-Time: {:02}:{:02}:{:02}, Battery capacity left: {:>5.1}%",
                        current_coverage_percent,
                        coverage_cell_count,
                        model.grid_cells_x * model.grid_cells_y - model.grid_cells_obstacles_count,
                        model.distance_covered,
                        model.segment_number,
                        model.sim_time_elapsed as u64 / 3600,
                        (model.sim_time_elapsed as u64 % 3600) / 60,
                        model.sim_time_elapsed as u64 % 60,
                        model.battery_charge_left
                    );
                } else {
                    print!(
                        "\rCoverage: {:>6.2}% ({:>7}/{:>7} cells covered), Distance: {:>6.2}, Bounces: {:>4}, Sim-Time: {:02}:{:02}:{:02}",
                        current_coverage_percent,
                        coverage_cell_count,
                        model.grid_cells_x * model.grid_cells_y - model.grid_cells_obstacles_count,
                        model.distance_covered,
                        model.segment_number,
                        model.sim_time_elapsed as u64 / 3600,
                        (model.sim_time_elapsed as u64 % 3600) / 60,
                        model.sim_time_elapsed as u64 % 60,
                    );
                }
                std::io::stdout().flush().unwrap();
            }
        }
    }
}
