use crate::cells::{calc_grid_coverage, mark_covered_cells};
use crate::collision::{BoundingBox, check_collision, collision_strategy};
use crate::model::{CoverageInfo, SimModel};
use rand::Rng;
use std::io::Write;

pub const FAILSAFE_TIME_LIMIT: f64 = 1_000_000.0; // 1 million seconds to prevent infinite loops

pub fn simulation_loop(
    sim_model: &mut SimModel,
    bounding_box: &BoundingBox,
    coverage_grid: &mut [Vec<CoverageInfo>],
    rng: &mut impl Rng,
) {
    // Current bounce count and coverage percentage
    let mut current_bounce_count = 0;
    let mut current_coverage_percent = 0.0;
    let mut coverage_count;
    let mut sim_time_elapsed = 0.0;

    // Initialize the circle position binding it to the bounding box
    let mut current_circle_x = bounding_box.limit_x(sim_model.start_x);
    let mut current_circle_y = bounding_box.limit_y(sim_model.start_y);

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
            sim_model.square_size,
            sim_model.width,
            sim_model.height,
            current_bounce_count,
            coverage_grid,
            sim_model.track_center,
            sim_model.parallel,
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
            bounding_box,
            &mut current_dir_x,
            &mut current_dir_y,
            &mut current_circle_x,
            &mut current_circle_y,
        );

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

        // Updated coverage only if it has been requested and in addition only every 20 sim steps as it is expensive
        // as it requries a full scan of he entire grid as we haven't implemented a more efficient way to track coverage
        // incrementally yet
        if sim_model.sim_steps % 20 == 0 {
            (coverage_count, current_coverage_percent) = calc_grid_coverage(coverage_grid, sim_model.parallel);
            if sim_model.show_progress {
                print!(
                    "\rCoverage: {current_coverage_percent:.2}% ({coverage_count}/{} cells covered) - Bounces: {current_bounce_count} - Sim-Time: {:02}:{:02}:{:02}",
                    sim_model.width * sim_model.height,
                    sim_time_elapsed as u64 / 3600,
                    (sim_time_elapsed as u64 % 3600) / 60,
                    sim_time_elapsed as u64 % 60,
                );
                std::io::stdout().flush().unwrap();
            }
        }
    }
    sim_model.bounce_count = current_bounce_count;
    sim_model.sim_time_elapsed = sim_time_elapsed;
}
