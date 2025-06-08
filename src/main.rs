use clap::Parser;
use std::collections::HashSet;

#[derive(Parser)]
#[command(author, version, about = "Grid coverage simulation")]
struct Args {
    /// Radius of the circle (default: 40)
    #[arg(short = 'r', long, default_value_t = 40.0)]
    radius: f64,

    /// Grid width (default: 100)
    #[arg(short = 'w', long, default_value_t = 100)]
    width: usize,

    /// Grid height (default: 100)
    #[arg(short = 'g', long, default_value_t = 100)]
    height: usize,

    /// Size of each grid square (default: 10)
    #[arg(short = 's', long, default_value_t = 10.0)]
    square_size: f64,

    /// Starting X coordinate for the circle center (default: 0)
    #[arg(short = 'x', long, default_value_t = 0.0)]
    start_x: f64,

    /// Starting Y coordinate for the circle center (default: 0)
    #[arg(short = 'y', long, default_value_t = 0.0)]
    start_y: f64,

    /// Movement velocity in units/second (default: 0.5)
    #[arg(short = 'v', long, default_value_t = 0.5)]
    velocity: f64,

    /// Direction X component (default: 1)
    #[arg(long, default_value_t = 1.0)]
    dir_x: f64,

    /// Direction Y component (default: 1)
    #[arg(long, default_value_t = 1.0)]
    dir_y: f64,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct GridCell {
    x: usize,
    y: usize,
}

fn main() {
    let args = Args::parse();
    
    // Normalize the direction vector
    let dir_length = (args.dir_x.powi(2) + args.dir_y.powi(2)).sqrt();
    let dir_x = args.dir_x / dir_length;
    let dir_y = args.dir_y / dir_length;
    
    // Calculate the simulation step size in terms of distance
    let step_size = 0.1; // Moving 0.1 units per step
    
    // Initialize the state
    let mut circle_pos_x = args.start_x;
    let mut circle_pos_y = args.start_y;
    let mut covered_cells: HashSet<GridCell> = HashSet::new();
    let mut time_elapsed = 0.0;
    
    // Determine the maximum possible coordinates to know when to stop
    let max_x = (args.width as f64) * args.square_size;
    let max_y = (args.height as f64) * args.square_size;
    
    // Run simulation until the circle moves out of the grid
    while circle_pos_x >= -args.radius && 
          circle_pos_x <= max_x + args.radius && 
          circle_pos_y >= -args.radius && 
          circle_pos_y <= max_y + args.radius {
        
        // Find all grid cells that are fully covered by the circle at the current position
        mark_covered_cells(
            circle_pos_x, 
            circle_pos_y, 
            args.radius, 
            args.square_size, 
            args.width, 
            args.height, 
            &mut covered_cells
        );
        
        // Move the circle
        circle_pos_x += dir_x * step_size;
        circle_pos_y += dir_y * step_size;
        
        // Update time
        time_elapsed += step_size / args.velocity;
    }
    
    // Print the grid
    print_grid(args.width, args.height, &covered_cells);
    
    // Print the time
    println!("Time elapsed: {:.2} seconds", time_elapsed);
}

/// Check if a grid cell is completely covered by the circle
fn is_cell_covered(
    circle_x: f64, 
    circle_y: f64, 
    radius: f64, 
    cell_x: usize, 
    cell_y: usize, 
    square_size: f64
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
    
    // A corner is inside the circle if its distance from the circle center is less than or equal to the radius
    corners.iter().all(|(x, y)| {
        let dx = x - circle_x;
        let dy = y - circle_y;
        dx*dx + dy*dy <= radius*radius
    })
}

/// Mark all grid cells completely covered by the circle
fn mark_covered_cells(
    circle_x: f64, 
    circle_y: f64, 
    radius: f64, 
    square_size: f64, 
    grid_width: usize, 
    grid_height: usize, 
    covered_cells: &mut HashSet<GridCell>
) {
    // Calculate the bounding box of the circle to optimize the search
    let min_x = ((circle_x - radius) / square_size).floor().max(0.0) as usize;
    let max_x = ((circle_x + radius) / square_size).ceil().min(grid_width as f64 - 1.0) as usize;
    let min_y = ((circle_y - radius) / square_size).floor().max(0.0) as usize;
    let max_y = ((circle_y + radius) / square_size).ceil().min(grid_height as f64 - 1.0) as usize;
    
    // Check all cells in the bounding box
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if is_cell_covered(circle_x, circle_y, radius, x, y, square_size) {
                covered_cells.insert(GridCell { x, y });
            }
        }
    }
}

/// Print the final grid
fn print_grid(width: usize, height: usize, covered_cells: &HashSet<GridCell>) {
    // Print from top to bottom
    for y in (0..height).rev() {
        for x in 0..width {
            if covered_cells.contains(&GridCell { x, y }) {
                print!("*");
            } else {
                print!("-");
            }
        }
        println!();
    }
}
