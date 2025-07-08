use crate::model::grid::Grid;

#[allow(clippy::collapsible_if)]
pub fn apply_circle_obstacle(
    grid: &mut Grid,
    center_x: f64,
    center_y: f64,
    radius: f64,
) {
    // Convert center to grid coordinates
    let grid_center_x = grid.coordinate_to_grid_x(center_x) as i32;
    let grid_center_y = grid.coordinate_to_grid_y(center_y) as i32;
    let cell_size = grid.get_cell_size();

    // Calculate the bounding box of the circle in grid coordinates
    let grid_radius = (radius / cell_size).ceil() as i32;

    // Check each cell in the bounding box if it falls within the circle
    for dx in -grid_radius..=grid_radius {
        for dy in -grid_radius..=grid_radius {
            let grid_x = grid_center_x + dx;
            let grid_y = grid_center_y + dy;

            if grid_x < 0 || grid_y < 0 {
                continue;
            }

            if grid_x >= grid.get_numcells().0 as i32 || grid_y >= grid.get_numcells().1 as i32
            {
                continue;
            }

            // Check if the cell is within the circle
            if is_cell_in_circle(
                grid,
                grid_x as usize,
                grid_y as usize,
                center_x,
                center_y,
                radius,
            ) {
                if let Some(cell) = grid.get_cell_mut(grid_x as usize, grid_y as usize) {
                    cell.set_as_obstacle();
                }
            }
        }
    }
}

fn is_cell_in_circle(
    grid: &Grid,
    grid_x: usize,
    grid_y: usize,
    center_x: f64,
    center_y: f64,
    radius: f64,
) -> bool {
    // Get real-world coordinates of cell corners
    let cell_corners = [
        grid.grid_to_coordinate(grid_x, grid_y),     // Bottom-left
        grid.grid_to_coordinate(grid_x + 1, grid_y), // Bottom-right
        grid.grid_to_coordinate(grid_x + 1, grid_y + 1), // Top-right
        grid.grid_to_coordinate(grid_x, grid_y + 1), // Top-left
    ];

    // If any corner is inside the circle, mark as obstacle
    for (corner_x, corner_y) in cell_corners {
        let dx = corner_x - center_x;
        let dy = corner_y - center_y;
        if dx * dx + dy * dy <= radius * radius {
            return true;
        }
    }

    false
}
