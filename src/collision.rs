use rand::Rng;

pub struct BoundingBox {
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
}

impl BoundingBox {
    fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
    pub fn init(width: usize, height: usize, radius: f64, square_size: f64) -> Self {
        let min_x = radius;
        let max_x = (width as f64) * square_size - radius;
        let min_y = radius;
        let max_y = (height as f64) * square_size - radius;
        Self::new(min_x, max_x, min_y, max_y)
    }
    pub fn limit_x(&self, x: f64) -> f64 {
        x.max(self.min_x).min(self.max_x)
    }
    pub fn limit_y(&self, y: f64) -> f64 {
        y.max(self.min_y).min(self.max_y)
    }
}

#[allow(clippy::too_many_arguments)]
pub fn check_collision(
    next_pos_x: f64,
    next_pos_y: f64,
    bb: &BoundingBox,
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
