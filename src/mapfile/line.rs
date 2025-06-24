use crate::{mapfile::polygon, model::grid::Grid};

pub fn apply_line_obstacle(grid: &mut Grid, points: &[[f64; 2]], width: f64) {
    if points.len() < 2 {
        eprintln!("Line must have at least 2 points");
        return;
    }

    let half_width = width / 2.0;

    // Process each line segment
    for segment in points.windows(2) {
        let start = segment[0];
        let end = segment[1];

        // Create a rectangle for this line segment
        let line_polygon = create_line_polygon(start, end, half_width);

        // Apply the polygon as an obstacle
        polygon::apply_polygon_obstacle(grid, &line_polygon);
    }
}

/// Create a polygon representing a line segment with specified width
fn create_line_polygon(
    start: [f64; 2],
    end: [f64; 2],
    half_width: f64,
) -> Vec<[f64; 2]> {
    let dx = end[0] - start[0];
    let dy = end[1] - start[1];

    // Calculate the length of the line segment
    let length = (dx * dx + dy * dy).sqrt();

    // Handle degenerate case where start and end are the same
    if length < f64::EPSILON {
        // Create a small square around the point
        return vec![
            [start[0] - half_width, start[1] - half_width],
            [start[0] + half_width, start[1] - half_width],
            [start[0] + half_width, start[1] + half_width],
            [start[0] - half_width, start[1] + half_width],
        ];
    }

    // Calculate the perpendicular unit vector (normal to the line)
    let perp_x = -dy / length * half_width;
    let perp_y = dx / length * half_width;

    // Create the four corners of the rectangle
    vec![
        [start[0] + perp_x, start[1] + perp_y], // Start point + perpendicular offset
        [start[0] - perp_x, start[1] - perp_y], // Start point - perpendicular offset
        [end[0] - perp_x, end[1] - perp_y],     // End point - perpendicular offset
        [end[0] + perp_x, end[1] + perp_y],     // End point + perpendicular offset
    ]
}
