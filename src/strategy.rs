use crate::model::SimModel;
use rand::Rng;
use crate::vector::Vector;

/// What to do when a collision is detected. 
/// The collision detection routin reverses the direction of the cutter so the current_dir is the direction of a perfect bounce
/// Here we check if the user enabled perturbation and if so, we apply a random perturbation angle to the current direction.
fn collision_strategy(
    model: &mut crate::model::SimModel,
    current_dir: &Vector,
    _current_circle: &Vector,
    rng: &mut impl Rng,
) -> (f64, f64) {
    // Generate random perturbation angle between -60 and 60 degrees
    let angle_perturbation = if model.perturb {
        rng.random_range(-std::f64::consts::FRAC_PI_3..std::f64::consts::FRAC_PI_3)
    } else {
        0.0
    };

    // Calculate the current angle
    let current_angle = current_dir.y.atan2(current_dir.x);

    // Apply the perturbation
    let new_angle = current_angle + angle_perturbation;

    (new_angle.cos(), new_angle.sin())
}

/// This strategy function gets called from the framework every simulation step
/// The information provided is:
/// - current direction of the cutting circle
/// - current position of the cutting circle
/// - whether a collision was detected in the last step or not. The collision point is +/- PI rad relative to the current direction, no information is provided about the exact collision point
/// - the simulation model containing all parameters and state
/// - a random number generator to use for random decisions
pub fn cutter_strategy(
    current_dir: &Vector,
    cutter_center_pos: &Vector,
    collision_detected: bool,
    model: &mut SimModel,
    rng: &mut impl Rng,
) -> (f64, f64) {
    match collision_detected {
        false => {
            let (mut dir_x, mut dir_y) = (current_dir.x, current_dir.y);
            if model.perturb_segment {
                // If perturbation is enabled, we can also perturb the direction randomly while moving in a straight line
                // This is done with a specified probability every square distance travelled
                // We use the step size to determine how many simulation steps we need to cover one square distance
                // This is only done if we are colliding with a boundary

                // How many sim steps to cover the width/height of the cell/square
                let sim_steps_per_cell =
                    (model.cell_size / model.step_size).ceil() as u64;

                if model.sim_steps % sim_steps_per_cell == 0
                    && rng.random_bool(model.perturb_segment_percent)
                {
                    // Perturb the direction randomly +/- PI radians
                    let perturb_angle =
                        rng.random_range(-std::f64::consts::PI..=std::f64::consts::PI);

                    let angle = (dir_y).atan2(dir_x) + perturb_angle;
                    (dir_x, dir_y) = (angle.cos(), angle.sin());
                }
            }

            (dir_x, dir_y)
        }
        true => {
            let (dir_x, dir_y) = collision_strategy(model, current_dir, cutter_center_pos, rng);
            
            (dir_x, dir_y)
        }
    }
}
