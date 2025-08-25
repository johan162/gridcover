// use crate::cells::{calc_grid_coverage, mark_covered_cells};
use crate::collision::is_grid_edge;
use crate::image::try_save_image;
use crate::model::SimModel;
use crate::strategy::cutter_strategy;
use crate::vector::Vector;
// use colored::Colorize;
use rand::Rng;
use std::io::Write;

pub const FAILSAFE_TIME_LIMIT: f64 = 7.0 * 24.0 * 3600.0; // 7 days in simulated time to prevent infinite loop

fn fast_inv_sqrt(x: f64) -> f64 {
    let i = 0x5FE6EB50C7B537A9 - (x.to_bits() >> 1);
    let y = f64::from_bits(i);
    y * (1.5 - 0.5 * x * y * y) // One Newton iteration for better accuracy
}

fn normalize_vector(v: &mut Vector) {
    let inv_length = fast_inv_sqrt(v.x * v.x + v.y * v.y);
    v.x *= inv_length;
    v.y *= inv_length;
}

fn print_progress(
    model: &SimModel,
    frame_counter: u64,
    current_coverage_percent: f64,
    coverage_cell_count: usize,
) {
    if model.show_progress {
        if model.battery_run_time > 0.0 {
            if model.generate_frames {
                print!(
                    "\rFrame: {:>06}, Coverage: {:>6.2}% ({:>7}/{:>7} cells covered), Distance: {:>6.2}, Bounces: {:>4}, Sim-Time: {:02}:{:02}:{:02}, Battery capacity left: {:>5.1}%",
                    frame_counter,
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
            }
        } else if model.generate_frames {
            print!(
                "\rFrame: {:>06}, Coverage: {:>6.2}% ({:>7}/{:>7} cells covered), Distance: {:>6.2}, Bounces: {:>4}, Sim-Time: {:02}:{:02}:{:02}",
                frame_counter,
                current_coverage_percent,
                coverage_cell_count,
                model.grid_cells_x * model.grid_cells_y - model.grid_cells_obstacles_count,
                model.distance_covered,
                model.segment_number,
                model.sim_time_elapsed as u64 / 3600,
                (model.sim_time_elapsed as u64 % 3600) / 60,
                model.sim_time_elapsed as u64 % 60,
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

fn handle_battery_charge(
    model: &mut SimModel,
    mut time_elapsed_since_last_charge: f64,
    rng: &mut impl Rng,
) -> f64 {
    // Check if we should consider battery run-time
    if model.battery_run_time > 0.0 {
        // If the battery run time is set, we need to check if we have reached it
        // Battery run time is in minutes and we have a constant power consumption

        model.battery_charge_left =
            100.0 - (time_elapsed_since_last_charge / (model.battery_run_time * 60.0)) * 100.0;

        if time_elapsed_since_last_charge > model.battery_run_time * 60.0 {
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
            time_elapsed_since_last_charge = 0.0;
            model.sim_time_elapsed += model.battery_charge_time * 60.0; // Add the charging time in seconds
            model.battery_charge_count += 1;
        }
    }
    time_elapsed_since_last_charge
}

struct SlippageModel {
    within_slippage: bool,
    current_distance: f64,
    adjustment_angle: f64,
    end_distance: f64,
    last_adjustment_distance: f64,
    last_activation_check: f64,
}

impl SlippageModel {
    fn new() -> Self {
        SlippageModel {
            within_slippage: false,
            current_distance: 0.0,
            adjustment_angle: 0.0,
            end_distance: 0.0,
            last_adjustment_distance: 0.0,
            last_activation_check: 0.0,
        }
    }
}

fn handle_wheel_slippage(
    model: &mut SimModel,
    slippage_model: &mut SlippageModel,
    current_dir: &mut Vector,
    rng: &mut impl Rng,
) {
    if model.wheel_slippage {
        if slippage_model.within_slippage {
            if model.distance_covered - slippage_model.last_adjustment_distance
                >= model.slippage_adjustment_step
            {
                slippage_model.last_adjustment_distance = model.distance_covered;

                current_dir.x -= current_dir.y * slippage_model.adjustment_angle;
                current_dir.y += current_dir.x * slippage_model.adjustment_angle;
                normalize_vector(current_dir);

                if slippage_model.current_distance >= slippage_model.end_distance {
                    // Stop slippage state after the slippage distance is covered
                    slippage_model.within_slippage = false;
                    slippage_model.current_distance = 0.0;
                    slippage_model.last_activation_check = model.distance_covered;
                }
            }
            slippage_model.current_distance += model.step_size;
        } else if model.distance_covered - slippage_model.last_activation_check
            >= model.slippage_check_activation_distance
        {
            slippage_model.last_activation_check = model.distance_covered;
            // If not within slippage range, we randomly decide to enter a slippage state
            if rng.random_range(0.0..1.0) < model.slippage_probability {
                slippage_model.within_slippage = true;

                let slippage_sign = if rng.random_bool(0.5) { -1.0 } else { 1.0 };
                let slippage_radius = rng
                    .random_range(model.slippage_radius_min..=model.slippage_radius_max)
                    * slippage_sign;
                slippage_model.adjustment_angle = model.slippage_adjustment_step / slippage_radius;

                slippage_model.end_distance =
                    rng.random_range(model.slippage_min_distance..=model.slippage_max_distance);

                current_dir.x -= current_dir.y * slippage_model.adjustment_angle;
                current_dir.y += current_dir.x * slippage_model.adjustment_angle;
                normalize_vector(current_dir);

                slippage_model.current_distance = 0.0;
                slippage_model.last_adjustment_distance = model.distance_covered;

                if model.verbosity > 2 {
                    println!(
                        "\nSlippage activated at distance: {:.4},  with angle: {:.4}, radius {:.4}, length:{:.4}",
                        slippage_model.last_adjustment_distance,
                        slippage_model.adjustment_angle.to_degrees(),
                        slippage_radius,
                        slippage_model.end_distance
                    );
                }
            }
        }
    }
}

struct InbalanceModel {
    adjustment_angle: f64,
    last_adjustment_distance: f64,
    radius: f64,
}

impl InbalanceModel {
    fn new() -> Self {
        InbalanceModel {
            adjustment_angle: 0.0,
            last_adjustment_distance: 0.0,
            radius: 0.0,
        }
    }

    fn init(&mut self, model: &mut SimModel, rng: &mut impl Rng) {
        // Generate randomly a -1 or a +1
        let inbalance_sign = if rng.random_bool(0.5) { -1.0 } else { 1.0 };
        self.radius = rng
            .random_range(model.wheel_inbalance_radius_min..=model.wheel_inbalance_radius_max)
            * inbalance_sign;
        model.wheel_inbalance_radius_used = self.radius;
        self.adjustment_angle = model.wheel_inbalance_adjustment_step / self.radius;
        self.last_adjustment_distance = 0.0;
    }
}

fn handle_wheel_inbalance(
    model: &mut SimModel,
    inbalance_model: &mut InbalanceModel,
    current_dir: &mut Vector,
) {
    // Update the wheel inbalance model
    if model.wheel_inbalance
        && model.distance_covered - inbalance_model.last_adjustment_distance
            >= model.wheel_inbalance_adjustment_step
    {
        inbalance_model.last_adjustment_distance = model.distance_covered;
        current_dir.x -= current_dir.y * inbalance_model.adjustment_angle;
        current_dir.y += current_dir.x * inbalance_model.adjustment_angle;
        normalize_vector(current_dir);

        if model.verbosity > 2 {
            println!(
                "\nInbalance activated at distance: {:.4},  with angle: {:.4}, radius {:.4}, length:{:.4}",
                inbalance_model.last_adjustment_distance,
                inbalance_model.adjustment_angle.to_degrees(),
                inbalance_model.radius,
                model.distance_covered
            );
        }
    }
}

pub fn simulation_loop(model: &mut SimModel, rng: &mut impl Rng) {
    // Both wheel slippage and inbalance are modelled as a slight change in the direction vector
    // we model this by multiplying the direction vector with a rotation matrix
    // and using the optimization that sin(a) = a , cos(a) = 1-(a^2)/2  for small angles.
    // Also realizing that a^2 is negligible for small angles, we can linearize the rotation
    // as:
    // vx' = vx * cos(a) - vy * sin(a)  => vx' = vx - vy * a
    // vy' = vx * sin(a) + vy * cos(a)  => vy' = vy + vx * a
    // where vx and vy are the components of the direction
    // vector, and a is the angle of rotation in radians.
    // We then normalize the direction vector after each adjustment to keep it a unit vector.
    // For this we use an approximation of the inverse square root
    // which is faster than the standard sqrt function.
    const ERROR_MSG: &str = "Failed to get grid. Internal BUG!";

    let mut current_coverage_percent = 0.0;
    let mut coverage_cell_count;
    let mut time_since_last_charge = 0.0;

    // Initialize the circle position binding it to the bounding box
    let mut cutter_center = Vector::new(
        model.bb.limit_x(model.start_x),
        model.bb.limit_y(model.start_y),
    );

    let mut current_dir = Vector::new(model.start_dir_x, model.start_dir_y);

    let total_cells = model.grid_cells_x * model.grid_cells_y;
    let one_percent_cells = total_cells / 100;
    let steps_per_cell = (model.cell_size / model.step_size).ceil() as usize;
    let steps_per_tenth_percent = (one_percent_cells * steps_per_cell / 10).min(1) as u64;
    let mut frame_counter: u64 = 0;
    let mut frame_image_numbering = 0;

    let mut slippage_model = SlippageModel::new();
    let mut inbalance_model = InbalanceModel::new();
    inbalance_model.init(model, rng);

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

        // Calculate the next position of the cutter center based on the current direction and step size
        cutter_center += current_dir * model.step_size;

        // Model an inbalance between the wheels on either side. We model this as a random turning radius
        handle_wheel_inbalance(model, &mut inbalance_model, &mut current_dir);

        // Simulate one side wheel slippage that will cause the cuttter to slightly alter its course
        // This is a simple model of slippage, where we randomly change the direction slightly
        // This is done to simulate a more realistic movement of the cutter
        handle_wheel_slippage(model, &mut slippage_model, &mut current_dir, rng);

        // Find and mark all grid cells that are fully covered by the circle at the current position
        model.grid.as_mut().expect(ERROR_MSG).mark_covered_cells(
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
            && model
                .grid
                .as_mut()
                .expect(ERROR_MSG)
                .collision_with_obstacle(&cutter_center, model.radius)
        {
            current_dir = -current_dir; // Reverse direction if we hit an obstacle
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

        current_dir = cutter_strategy(&current_dir, &cutter_center, collision_detected, model, rng);

        // Update time in the simulation
        // sim_time_elapsed is in seconds, so we divide the step size by the velocity to get the time for this step
        // This assumes velocity is in units/second
        model.sim_time_elapsed += model.step_size / model.velocity;
        time_since_last_charge += model.step_size / model.velocity;

        time_since_last_charge = handle_battery_charge(model, time_since_last_charge, rng);

        if model.sim_steps == 1 || 
            model.sim_steps % steps_per_tenth_percent == 0 {
            (coverage_cell_count, current_coverage_percent) = model.grid.as_ref().expect(ERROR_MSG).get_coverage();

            print_progress(
                model,
                frame_counter,
                current_coverage_percent,
                coverage_cell_count,
            );
        }

        if model.generate_frames && (model.sim_steps % model.steps_per_frame) == 0 {
            if frame_counter % model.animation_speedup == 0 {
                let frame_filename = format!(
                    "{}/frame_{:07}.png",
                    model.frames_dir, frame_image_numbering
                );
                frame_image_numbering += 1;
                try_save_image(model, Some(frame_filename));
            }
            frame_counter += 1;
        }
    }
}
