use crate::model::{coverageinfo::CoverageInfo, cuttertype::CutterType, SimModel};
use crate::vector::Vector;
use rayon::prelude::*;

// Ugly. We set the times_visited counter to a really high value we are very, very unlikely to reach
// to indicate that this is a center point of the circle. This is used to color the center point differently.pub
pub const CENTERPOINT_MAGIC_CONSTANT: usize = 9999;

/// Check if a grid cell is completely covered by the circle
#[allow(clippy::too_many_arguments)]
pub fn is_cell_covered(
    circle_x: f64,
    circle_y: f64,
    radius: f64,
    blade_len: f64,
    cell_x: usize,
    cell_y: usize,
    square_size: f64,
    cutter_type: CutterType,
) -> bool {
    // Calculate the coordinates of the four corners of the cell
    let cell_left = cell_x as f64 * square_size;
    let cell_right = cell_left + square_size;
    let cell_bottom = cell_y as f64 * square_size;
    let cell_top = cell_bottom + square_size;

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
            let dx = x - circle_x;
            let dy = y - circle_y;
            dx * dx + dy * dy <= radius * radius
        }) && corners.iter().all(|(x, y)| {
            let dx = x - circle_x;
            let dy = y - circle_y;
            dx * dx + dy * dy >= radius_inner * radius_inner
        })
    } else {
        // A cell is covered by the circle if all corners are within the radius
        corners.iter().all(|(x, y)| {
            let dx = x - circle_x;
            let dy = y - circle_y;
            dx * dx + dy * dy <= radius * radius
        })
    }
}

pub fn mark_covered_cells(cutter_center: &Vector, model: &mut SimModel) {
    if model.parallel {
        mark_covered_cells_parallel(cutter_center, model);
    } else {
        mark_covered_cells_linear(cutter_center, model);
    }
}

/// Mark all grid cells completely covered by the circle
fn mark_covered_cells_linear(cutter_center: &Vector, model: &mut SimModel) {
    // Calculate the bounding box of the circle to optimize the search
    let min_x = ((cutter_center.x - model.radius) / model.square_size)
        .floor()
        .max(0.0) as usize;
    let max_x = ((cutter_center.x + model.radius) / model.square_size)
        .ceil()
        .min(model.grid_cells_x as f64 - 1.0) as usize;
    let min_y = ((cutter_center.y - model.radius) / model.square_size)
        .floor()
        .max(0.0) as usize;
    let max_y = ((cutter_center.y + model.radius) / model.square_size)
        .ceil()
        .min(model.grid_cells_y as f64 - 1.0) as usize;

    // Use enumeration to access rows directly
    for (y_offset, row) in model
        .coverage_grid
        .iter_mut()
        .enumerate()
        .skip(min_y)
        .take(max_y - min_y + 1)
    {
        let y = y_offset; // y is the actual y-coordinate as we're skipping to min_y

        // Use enumeration to access cells directly
        for (x_offset, cell) in row
            .iter_mut()
            .enumerate()
            .skip(min_x)
            .take(max_x - min_x + 1)
        {
            let x = x_offset; // x is the actual x-coordinate as we're skipping to min_x

            if is_cell_covered(
                cutter_center.x,
                cutter_center.y,
                model.radius,
                model.blade_len,
                x,
                y,
                model.square_size,
                model.cutter_type,
            ) {
                // Only mark the cell if it hasn't been covered before
                if !cell.covered {
                    *cell = CoverageInfo {
                        covered: true,
                        bounce_number: model.bounce_count,
                        times_visited: 1,
                    };
                } else {
                    // If we are still on the same leg we don't increase the bounce count
                    // but we increase the times visited counter
                    if model.bounce_count != cell.bounce_number {
                        if !model.track_center
                            || cell.times_visited != CENTERPOINT_MAGIC_CONSTANT
                        {
                            cell.times_visited += 1;
                        }
                        cell.bounce_number = model.bounce_count; // Update to the latest bounce number
                    }
                }
            }
        }
    }

    if model.track_center {
        // Mark the center of the circle square with a 9
        let center_x = (cutter_center.x / model.square_size).round() as usize;
        let center_y = (cutter_center.y / model.square_size).round() as usize;
        if center_x < model.grid_cells_x && center_y < model.grid_cells_y {
            model.coverage_grid[center_y][center_x].covered = true;
            model.coverage_grid[center_y][center_x].bounce_number = model.bounce_count;
            model.coverage_grid[center_y][center_x].times_visited = CENTERPOINT_MAGIC_CONSTANT;
        }
    }
}

fn mark_covered_cells_parallel(cutter_center: &Vector, model: &mut SimModel) {
    let min_x = ((cutter_center.x - model.radius) / model.square_size)
        .floor()
        .max(0.0) as usize;
    let max_x = ((cutter_center.x + model.radius) / model.square_size)
        .ceil()
        .min(model.grid_cells_x as f64 - 1.0) as usize;
    let min_y = ((cutter_center.y - model.radius) / model.square_size)
        .floor()
        .max(0.0) as usize;
    let max_y = ((cutter_center.y + model.radius) / model.square_size)
        .ceil()
        .min(model.grid_cells_y as f64 - 1.0) as usize;

    // Parallelize over y (rows)
    model.coverage_grid[min_y..=max_y]
        .par_iter_mut()
        .enumerate()
        .for_each(|(dy, row)| {
            let y = min_y + dy;

            // Use filter_map to process only cells within our x range and get mutable references
            for (x, cell) in row
                .iter_mut()
                .enumerate()
                .skip(min_x)
                .take(max_x - min_x + 1)
            {
                if is_cell_covered(
                    cutter_center.x,
                    cutter_center.y,
                    model.radius,
                    model.blade_len,
                    x,
                    y,
                    model.square_size,
                    model.cutter_type,
                ) {
                    if !cell.covered {
                        *cell = CoverageInfo {
                            covered: true,
                            bounce_number: model.bounce_count,
                            times_visited: 1,
                        };
                    } else if model.bounce_count != cell.bounce_number {
                        if !model.track_center
                            || cell.times_visited != CENTERPOINT_MAGIC_CONSTANT
                        {
                            cell.times_visited += 1;
                        }
                        cell.bounce_number = model.bounce_count;
                    }
                }
            }
        });

    if model.track_center {
        // Mark the center of the circle square with a 9
        let center_x = (cutter_center.x / model.square_size).round() as usize;
        let center_y = (cutter_center.y / model.square_size).round() as usize;
        if center_x < model.grid_cells_x && center_y < model.grid_cells_y {
            model.coverage_grid[center_y][center_x].covered = true;
            model.coverage_grid[center_y][center_x].bounce_number = model.bounce_count;
            model.coverage_grid[center_y][center_x].times_visited = CENTERPOINT_MAGIC_CONSTANT;
        }
    }
}

/// Check if the circle is within or partially within the grid boundaries
#[allow(dead_code)]
pub fn is_circle_in_grid(
    circle_x: f64,
    circle_y: f64,
    radius: f64,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> bool {
    // Check if circle is NOT completely outside the grid
    !(circle_x + radius < min_x
        || circle_x - radius > max_x
        || circle_y + radius < min_y
        || circle_y - radius > max_y)
}

/// Print the final grid with color-coded symbols based on bounce number
pub fn print_grid(width: usize, height: usize, coverage_grid: &[Vec<CoverageInfo>]) {
    use colored::*;

    // Define bounce characters to make each bounce visibly distinct
    const BOUNCE_CHARS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    // Print from top to bottom
    for y in (0..height).rev() {
        for x in 0..width {
            let info = &coverage_grid[y][x];
            if info.covered {
                // Get the bounce character, cycling through the available chars
                let bounce_char = BOUNCE_CHARS[info.bounce_number % BOUNCE_CHARS.len()];

                // Color based on bounce number
                let colored_str = match info.bounce_number {
                    0 => bounce_char.to_string().red(),
                    1 => bounce_char.to_string().green(),
                    2 => bounce_char.to_string().yellow(),
                    3 => bounce_char.to_string().blue(),
                    4 => bounce_char.to_string().magenta(),
                    _ => bounce_char.to_string().cyan(),
                };
                // Special case for the center point
                if info.times_visited == CENTERPOINT_MAGIC_CONSTANT {
                    // Center point, use white color
                    print!("{}", "**".bold().white());
                } else {
                    // Normal bounce cell
                    print!("{colored_str}{colored_str}");
                }
            } else {
                print!("--");
            }
        }
        println!();
    }

    // Print a legend
    println!("\nLegend:");
    println!("'-': Cell not covered");
    println!("'0-9': Cell covered during bounce 0-9 (wraps around for higher bounces)");
}

pub fn calc_grid_coverage(coverage_grid: &[Vec<CoverageInfo>], parallel: bool) -> (usize, f64) {
    if parallel {
        calc_grid_coverage_parallel(coverage_grid)
    } else {
        calc_grid_coverage_linear(coverage_grid)
    }
}

fn calc_grid_coverage_linear(coverage_grid: &[Vec<CoverageInfo>]) -> (usize, f64) {
    let mut covered_count = 0;
    let total_cells = coverage_grid.len() * coverage_grid[0].len();

    for row in coverage_grid.iter() {
        for cell in row.iter() {
            if cell.covered {
                covered_count += 1;
            }
        }
    }

    let coverage_percent = (covered_count as f64 / total_cells as f64) * 100.0;
    (covered_count, coverage_percent)
}

pub fn calc_grid_coverage_parallel(coverage_grid: &[Vec<CoverageInfo>]) -> (usize, f64) {
    use rayon::prelude::*;

    let total_cells = coverage_grid.len() * coverage_grid[0].len();

    // Parallel count of covered cells
    let covered_count = coverage_grid
        .par_iter()
        .map(|row| row.iter().filter(|cell| cell.covered).count())
        .sum();

    let coverage_percent = (covered_count as f64 / total_cells as f64) * 100.0;
    (covered_count, coverage_percent)
}
