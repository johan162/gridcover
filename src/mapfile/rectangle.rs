use crate::model::grid::Grid;

// Implementation of obstacle application functions
pub fn apply_rectangle_obstacle(
    grid: &mut Grid,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    // Convert real coordinates to grid coordinates
    let (grid_x, grid_y) = grid.coordinate_to_grid(x, y);
    let cell_size = grid.get_cell_size();
    let grid_width = (width / cell_size).ceil() as usize;
    let grid_height = (height / cell_size).ceil() as usize;

    // Mark cells as obstacles
    for dx in 0..grid_width {
        for dy in 0..grid_height {
            if let Some(cell) = grid.get_cell_mut(grid_x + dx, grid_y + dy) {
                cell.set_as_obstacle();
            }
        }
    }
}
