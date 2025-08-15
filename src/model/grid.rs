use crate::model::{coverageinfo::CoverageInfo, cuttertype::CutterType, quadtree::QuadTree};
use crate::vector::Vector;

#[derive(Debug, Clone)]
pub enum Cell {
    Empty,
    Covered(CoverageInfo),
    CenterPoint(CoverageInfo),
    Obstacle,
}

impl Cell {
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }

    #[allow(dead_code)]
    pub fn is_covered(&self) -> bool {
        matches!(self, Cell::Covered(_))
    }

    pub fn is_obstacle(&self) -> bool {
        matches!(self, Cell::Obstacle)
    }

    pub fn set_as_obstacle(&mut self) {
        *self = Cell::Obstacle;
    }

    pub fn set_as_covered(&mut self, segment_number: usize) {
        *self = Cell::Covered(CoverageInfo::new(segment_number, 1));
    }
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub cells: Vec<Vec<Cell>>,
    pub cell_size: f64,
    pub cells_x: usize,
    pub cells_y: usize,
    pub covered_cells: usize,
    pub cells_obstacles_count: usize,
    pub quadtree: Option<QuadTree>,
    pub num_detailed_collision_checks: usize,
    pub use_quad_tree: bool,
}

impl Grid {
    pub fn new(grid_cells_x: usize, grid_cells_y: usize, cell_size: f64) -> Self {
        let cells = vec![vec![Cell::Empty; grid_cells_y]; grid_cells_x];

        Grid {
            cells,
            cell_size,
            cells_x: grid_cells_x,
            cells_y: grid_cells_y,
            covered_cells: 0,
            cells_obstacles_count: 0,
            quadtree: None,
            num_detailed_collision_checks: 0,
            use_quad_tree: false,
        }
    }

    /// Initialize the spatial index for the grid using a quad-tree
    pub fn init_spatial_index(&mut self, cutter_radius: f64, min_qnode_size: f64) {
        let quad_tree = QuadTree::build_from_grid(self, cutter_radius, min_qnode_size);
        self.quadtree = Some(quad_tree);
    }

    /// Save the spatial index to a file
    pub fn save_spatial_index<P: AsRef<std::path::Path>>(&self, path: P) -> std::io::Result<()> {
        if let Some(tree) = &self.quadtree {
            tree.save_to_file(path)
        } else {
            Ok(()) // No spatial index to save
        }
    }

    #[allow(dead_code)]
    /// Load the spatial index from a file
    pub fn load_spatial_index<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> std::io::Result<()> {
        self.quadtree = Some(QuadTree::load_from_file(path)?);
        Ok(())
    }

    pub fn get_numcells(&self) -> (usize, usize) {
        (self.cells_x, self.cells_y)
    }

    pub fn get_cell_size(&self) -> f64 {
        self.cell_size
    }

    #[allow(dead_code)]
    pub fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        if x < self.cells_x && y < self.cells_y {
            Some(&self.cells[x][y])
        } else {
            None
        }
    }

    // Count the number of cells with an obstacle
    pub fn update_obstacle_cells_count(&mut self) {
        self.cells_obstacles_count = self
            .cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|cell| cell.is_obstacle())
            .count();
    }

    pub fn get_cell_iter(&self) -> impl Iterator<Item = &Cell> {
        self.cells.iter().flat_map(|row| row.iter())
    }

    #[allow(dead_code)]
    pub fn get_cell_iter_mut(&mut self) -> impl Iterator<Item = &mut Cell> {
        self.cells.iter_mut().flat_map(|row| row.iter_mut())
    }

    pub fn get_cell_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x < self.cells_x && y < self.cells_y {
            Some(&mut self.cells[x][y])
        } else {
            None
        }
    }

    /// The the maximum cell visited number in the grid
    pub fn get_max_visited_number(&self) -> usize {
        self.get_cell_iter()
            .filter_map(|cell| match cell {
                Cell::Covered(info) => Some(info.times_visited),
                _ => None,
            })
            .max()
            .unwrap_or(0)
    }

    /// Get the minimum cell visited number in the grid
    pub fn get_min_visited_number(&self) -> usize {
        self.get_cell_iter()
            .filter_map(|cell| match cell {
                Cell::Covered(info) => Some(info.times_visited),
                _ => None,
            })
            .min()
            .unwrap_or(0)
    }

    /// Convert world coordinates to grid coordinates
    /// Always make this inline
    #[inline]
    pub fn world_coordinate_to_grid_x(&self, x: f64) -> usize {
        (x / self.cell_size).floor() as usize
    }

    /// Convert world coordinates to grid coordinates
    /// Always make this inline
    #[inline]
    pub fn world_coordinate_to_grid_y(&self, y: f64) -> usize {
        (y / self.cell_size).floor() as usize
    }

    /// Convert world coordinates to grid coordinates
    /// Always make this inline
    #[inline]
    pub fn world_coordinate_to_grid(&self, x: f64, y: f64) -> (usize, usize) {
        let grid_x = self.world_coordinate_to_grid_x(x);
        let grid_y = self.world_coordinate_to_grid_y(y);
        (grid_x, grid_y)
    }

    /// Convert grid coordinates to world coordinates
    /// Always make this inline
    #[inline]
    pub fn grid_to_world_coordinate(&self, grid_x: usize, grid_y: usize) -> (f64, f64) {
        let x = grid_x as f64 * self.cell_size;
        let y = grid_y as f64 * self.cell_size;
        (x, y)
    }

    // pub fn has_obstacle(&self, x: f64, y: f64) -> bool {
    //     let grid_x = self.coordinate_to_grid_x(x);
    //     let grid_y = self.coordinate_to_grid_y(y);

    //     if let Some(cell) = self.get_cell(grid_x, grid_y) {
    //         return cell.is_obstacle();
    //     }
    //     false
    // }

    /// Check if there is an obstacle in a bounding box determined by the given radius around the given center point.
    #[allow(clippy::collapsible_if)]
    pub fn collision_with_obstacle(&mut self, center: &Vector, radius: f64) -> bool {
        // First check the spatial index if available
        if self.use_quad_tree {
            if let Some(quad_tree) = &self.quadtree {
                if !quad_tree.might_have_collision(center.x, center.y, radius) {
                    return false; // No collision possible in this area
                }
            }
        }

        self.num_detailed_collision_checks += 1;

        // If no spatial index or quad-tree indicates possible collision,
        // perform detailed collision check
        let grid_radius = (radius / self.cell_size).ceil() as i32;
        let grid_center_x = self.world_coordinate_to_grid_x(center.x) as i32;
        let grid_center_y = self.world_coordinate_to_grid_y(center.y) as i32;

        for dx in -grid_radius..=grid_radius {
            for dy in -grid_radius..=grid_radius {
                let grid_cell_x = grid_center_x + dx;
                let grid_cell_y = grid_center_y + dy;
                if grid_cell_x < 0 || grid_cell_y < 0 {
                    continue;
                }
                if let Some(cell) = self.get_cell(grid_cell_x as usize, grid_cell_y as usize) {
                    if cell.is_obstacle() {
                        return true;
                    }
                }
            }
        }
        false
    }

    // pub fn get_cell_at_coordinates(&self, x: f64, y: f64) -> Option<&Cell> {
    //     let grid_x = self.coordinate_to_grid_x(x);
    //     let grid_y = self.coordinate_to_grid_y(y);
    //     self.get_cell(grid_x, grid_y)
    // }

    // pub fn get_cell_at_coordinates_mut(&mut self, x: f64, y: f64) -> Option<&mut Cell> {
    //     let grid_x = self.coordinate_to_grid_x(x);
    //     let grid_y = self.coordinate_to_grid_y(y);
    //     self.get_cell_mut(grid_x, grid_y)
    // }

    pub fn get_coverage_count(&self) -> usize {
        self.covered_cells
    }

    #[allow(dead_code)]
    pub fn get_obstacle_count(&self) -> usize {
        self.cells_obstacles_count
    }

    pub fn get_coverage_percent(&self) -> f64 {
        if self.cells_x == 0 || self.cells_y == 0 {
            return 0.0;
        }
        (self.covered_cells as f64
            / (self.cells_x * self.cells_y - self.cells_obstacles_count) as f64)
            * 100.0
    }

    pub fn get_coverage(&self) -> (usize, f64) {
        let count = self.get_coverage_count();
        let percent = self.get_coverage_percent();
        (count, percent)
    }

    /// Check if a grid cell is completely covered by the circle
    #[allow(clippy::too_many_arguments)]
    pub fn is_cell_covered(
        &self,
        center: &Vector,
        radius: f64,
        blade_len: f64,
        grid_cell_x: usize,
        grid_cell_y: usize,
        cutter_type: CutterType,
    ) -> bool {
        // Calculate the coordinates of the four corners of the cell
        let cell_left = grid_cell_x as f64 * self.cell_size;
        let cell_right = cell_left + self.cell_size;
        let cell_bottom = grid_cell_y as f64 * self.cell_size;
        let cell_top = cell_bottom + self.cell_size;

        // Check if all four corners are inside the circle
        let corners = [
            (cell_left, cell_bottom),  // Bottom-left
            (cell_right, cell_bottom), // Bottom-right
            (cell_left, cell_top),     // Top-left
            (cell_right, cell_top),    // Top-right
        ];

        if cutter_type == CutterType::Blade {
            let radius_inner = radius - blade_len;
            // A cell is covered by the knife blade if it is within the outer and inner radius
            corners.iter().all(|(x, y)| {
                let dx = x - center.x;
                let dy = y - center.y;
                dx * dx + dy * dy <= radius * radius
            }) && corners.iter().all(|(x, y)| {
                let dx = x - center.x;
                let dy = y - center.y;
                dx * dx + dy * dy >= radius_inner * radius_inner
            })
        } else {
            // A cell is covered by the circle if all corners are within the radius
            corners.iter().all(|(x, y)| {
                let dx = x - center.x;
                let dy = y - center.y;
                dx * dx + dy * dy <= radius * radius
            })
        }
    }

    pub fn mark_covered_cells(
        &mut self,
        center: &Vector,
        radius: f64,
        segment_number: usize,
        blade_len: f64,
        cutter_type: CutterType,
        track_center: bool,
    ) {
        let grid_radius = (radius / self.cell_size).ceil() as i32;
        let grid_center_x = self.world_coordinate_to_grid_x(center.x) as i32;
        let grid_center_y = self.world_coordinate_to_grid_y(center.y) as i32;

        for dx in -grid_radius..=grid_radius {
            for dy in -grid_radius..=grid_radius {
                let grid_cell_x = grid_center_x + dx;
                let grid_cell_y = grid_center_y + dy;
                if grid_cell_x < 0 || grid_cell_y < 0 {
                    continue;
                }

                #[allow(clippy::collapsible_if)]
                if self.is_cell_covered(
                    center,
                    radius,
                    blade_len,
                    grid_cell_x as usize,
                    grid_cell_y as usize,
                    cutter_type,
                ) {
                    if let Some(cell) =
                        self.get_cell_mut(grid_cell_x as usize, grid_cell_y as usize)
                    {
                        match cell {
                            Cell::Empty => {
                                cell.set_as_covered(segment_number);
                                self.covered_cells += 1;
                            }
                            Cell::Covered(existing_coverage) => {
                                if segment_number != existing_coverage.segment_number {
                                    existing_coverage.times_visited += 1;
                                    existing_coverage.segment_number = segment_number;
                                }
                            }
                            Cell::CenterPoint(existing_coverage) => {
                                if segment_number != existing_coverage.segment_number {
                                    existing_coverage.times_visited += 1;
                                    existing_coverage.segment_number = segment_number;
                                }
                            }
                            Cell::Obstacle => {
                                eprint!(
                                    "Attempted to cover a cell marked as an obstacle at ({grid_cell_x}, {grid_cell_y}), ({:.1}, {:.1})",
                                    grid_cell_x as f64 * self.cell_size,
                                    grid_cell_y as f64 * self.cell_size
                                );
                                panic!("Attempted to cover a cell marked as an obstacle");
                            }
                        }
                    }
                }
            }
        }

        // Handle the center point separately
        #[allow(clippy::collapsible_if)]
        if track_center {
            if let Some(cell) = self.get_cell_mut(grid_center_x as usize, grid_center_y as usize) {
                match cell {
                    Cell::Empty => {
                        *cell = Cell::CenterPoint(CoverageInfo::new(segment_number, 1));
                    }
                    Cell::Covered(existing_coverage) => {
                        *cell = Cell::CenterPoint(CoverageInfo::new(
                            existing_coverage.segment_number,
                            existing_coverage.times_visited,
                        ));
                    }
                    Cell::CenterPoint(existing_coverage) => {
                        if segment_number != existing_coverage.segment_number {
                            existing_coverage.times_visited += 1;
                            existing_coverage.segment_number = segment_number;
                        }
                    }
                    Cell::Obstacle => {
                        panic!("Attempted to mark center point in a cell marked as an obstacle");
                    }
                }
            }
        }
    }
}
