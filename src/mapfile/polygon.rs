use crate::model::grid::Grid;

/// Apply polygon obstacle using point-in-polygon algorithm
#[allow(clippy::collapsible_if)]
pub fn apply_polygon_obstacle(grid: &mut Grid, points: &[[f64; 2]]) {
    if points.len() < 3 {
        eprintln!("Polygon must have at least 3 points");
        return;
    }

    // Find bounding box of the polygon to limit our search area
    let (min_x, max_x, min_y, max_y) = get_polygon_bounds(points);

    // Convert to grid coordinates
    let grid_min_x = grid.coordinate_to_grid_x(min_x).saturating_sub(1);
    let grid_max_x = (grid.coordinate_to_grid_x(max_x) + 1).min(grid.cells_x);
    let grid_min_y = grid.coordinate_to_grid_y(min_y).saturating_sub(1);
    let grid_max_y = (grid.coordinate_to_grid_y(max_y) + 1).min(grid.cells_y);

    // Check each cell in the bounding box
    for grid_y in grid_min_y..grid_max_y {
        for grid_x in grid_min_x..grid_max_x {
            if is_cell_in_polygon(grid, grid_x, grid_y, points) {
                if let Some(cell) = grid.get_cell_mut(grid_x, grid_y) {
                    cell.set_as_obstacle();
                }
            }
        }
    }
}

/// Check if a grid cell is inside the polygon
fn is_cell_in_polygon(
    grid: &Grid,
    grid_x: usize,
    grid_y: usize,
    points: &[[f64; 2]],
) -> bool {
    // Get the center point of the cell

    let (cell_x, cell_y) = grid.grid_to_coordinate(grid_x, grid_y);
    let cell_center_x = cell_x + grid.cell_size / 2.0;
    let cell_center_y = cell_y + grid.cell_size / 2.0;

    // Use point-in-polygon algorithm (ray casting)
    point_in_polygon(cell_center_x, cell_center_y, points)
}

/// Ray casting algorithm to determine if a point is inside a polygon
fn point_in_polygon(x: f64, y: f64, polygon: &[[f64; 2]]) -> bool {
    let n = polygon.len();
    let mut inside = false;

    let mut j = n - 1;
    for i in 0..n {
        let xi = polygon[i][0];
        let yi = polygon[i][1];
        let xj = polygon[j][0];
        let yj = polygon[j][1];

        // Check if point is on the same horizontal line as the edge
        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }

    inside
}

/// Get the bounding box of a polygon
fn get_polygon_bounds(points: &[[f64; 2]]) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for point in points {
        min_x = min_x.min(point[0]);
        max_x = max_x.max(point[0]);
        min_y = min_y.min(point[1]);
        max_y = max_y.max(point[1]);
    }

    (min_x, max_x, min_y, max_y)
}
