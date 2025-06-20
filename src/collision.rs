use crate::vector::Vector;
use crate::model::boundingbox::BoundingBox;

/// The collision check examines if the next position of the circle center
/// collides with the boundaries defined by the bounding box (bb).
/// The bounding bos´x is reduced with the radius compared with the absolute boundaries to account for that a collsion happens when the outside of the circle
/// reaches the boundary, not the center of the circle.
pub fn check_collision(
    cutter_center_pos: &Vector,
    bb: &BoundingBox,
    dir: &mut Vector,
) -> bool {
    let mut collision_detected = false;
    // Collision with left or right boundary
    if cutter_center_pos.x < bb.min_x || cutter_center_pos.x > bb.max_x {
        dir.x = -dir.x; // Reverse x direction
        collision_detected = true;
    }

    // Collision with bottom or top boundary
    if cutter_center_pos.y < bb.min_y || cutter_center_pos.y > bb.max_y {
        dir.y = -dir.y; // Reverse y direction
        collision_detected = true;
    }
    collision_detected
}
