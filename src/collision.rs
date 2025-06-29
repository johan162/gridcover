use crate::model::boundingbox::BoundingBox;
use crate::vector::Vector;

/// Check if we are driving into a grid edge and reverse the direction if so.
/// Returns true if a grid edge was hit, false otherwise.
pub fn is_grid_edge(cutter_center_pos: &Vector, bb: &BoundingBox, dir: &mut Vector) -> bool {
    let mut grid_edge = false;
    // Collision with left or right boundary
    if cutter_center_pos.x < bb.min_x || cutter_center_pos.x > bb.max_x {
        dir.x = -dir.x; // Reverse x direction
        grid_edge = true;
    }

    // Collision with bottom or top boundary
    if cutter_center_pos.y < bb.min_y || cutter_center_pos.y > bb.max_y {
        dir.y = -dir.y; // Reverse y direction
        grid_edge = true;
    }
    grid_edge
}
