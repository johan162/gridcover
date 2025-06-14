use rand::Rng;


#[allow(clippy::too_many_arguments)]
pub fn check_collision(
    next_pos_x: f64,
    next_pos_y: f64,
    bb: &crate::model::BoundingBox,
    dir_x: &mut f64,
    dir_y: &mut f64,
    circle_pos_x: &mut f64,
    circle_pos_y: &mut f64,
) -> bool {
    let mut collision_detected = false;
    // Collision with left or right boundary
    if next_pos_x < bb.min_x || next_pos_x > bb.max_x {
        *dir_x = -*dir_x; // Reverse x direction
        collision_detected = true;
        // Adjust position to stay within boundaries
        *circle_pos_x = bb.limit_x(*circle_pos_x);
    }

    // Collision with bottom or top boundary
    if next_pos_y < bb.min_y || next_pos_y > bb.max_y {
        *dir_y = -*dir_y; // Reverse y direction
        collision_detected = true;
        // Adjust position to stay within boundaries
        *circle_pos_y = bb.limit_y(*circle_pos_y);
    }
    collision_detected
}

pub fn collision_strategy(
    perturb: bool,
    dir_x: f64,
    dir_y: f64,
    _circle_pos_x: &mut f64,
    _circle_pos_y: &mut f64,
    rng: &mut impl Rng,
) -> (f64, f64) {

    // Generate random perturbation angle between -60 and 60 degrees
    let angle_perturbation = if perturb {
        rng.random_range(-std::f64::consts::FRAC_PI_3..std::f64::consts::FRAC_PI_3)
    } else {
        0.0
    };

    // Calculate the current angle
    let current_angle = dir_y.atan2(dir_x);

    // Apply the perturbation
    let new_angle = current_angle + angle_perturbation;

    (new_angle.cos(), new_angle.sin())
}
