use crate::model::SimModel;
use crate::model::grid::Cell;
use colored::Colorize;

pub fn try_save_image(model: &SimModel, override_filename: Option<String>) {
    if model.image_file_name.is_some() || override_filename.is_some() {
        if let Err(err) = save_grid_image(model, override_filename) {
            eprintln!(
                "{} {}",
                "Error saving image:".color(colored::Color::Red).bold(),
                err
            );
        }
    }
}

/// Create a PNG image of the coverage grid with colored squares
fn save_grid_image(
    model: &crate::model::SimModel,
    override_filename: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert mm to pixels using DPI (Dots Per Inch)
    let pixels_per_mm = model.dpi as f64 / 25.4;

    let mut base_img_width_pixels = (model.image_width_mm as f64 * pixels_per_mm).round() as u32;
    let mut base_img_height_pixels = (model.image_height_mm as f64 * pixels_per_mm).round() as u32;

    if base_img_height_pixels < model.grid_cells_y as u32 {
        eprintln!(
            "{} {} mm",
            "Warning! Adjusting image height to fit grid height. New height at given DPI:"
                .yellow()
                .bold(),
            (model.grid_cells_y as u32 + 1) / pixels_per_mm as u32
        );
        base_img_height_pixels = model.grid_cells_y as u32 + 1;
    }

    if base_img_width_pixels < model.grid_cells_x as u32 {
        eprintln!(
            "{} {} mm",
            "Warning! Adjusting image width to fit grid. New width at given DPI:"
                .yellow()
                .bold(),
            (model.grid_cells_x as u32 + 1) / pixels_per_mm as u32
        );
        base_img_width_pixels = model.grid_cells_x as u32 + 1;
    }

    // Calculate cell size to ensure perfect squares
    // Take the smaller dimension to make sure image fits within requested size
    let cell_size = std::cmp::min(
        base_img_width_pixels / model.grid_cells_x as u32,
        base_img_height_pixels / model.grid_cells_y as u32,
    );

    // Recalculate image dimensions using the uniform cell size
    let img_width = cell_size * model.grid_cells_x as u32;
    let img_height = cell_size * model.grid_cells_y as u32;

    // Create a new RGB image buffer
    let mut img = image::RgbImage::new(img_width, img_height);

    // Define colors used in grid
    const GRID_BACKGROUND_COLOR: [u8; 3] = [150, 150, 150]; // Dark gray
    const GRID_LINE_COLOR: [u8; 3] = [0, 0, 0]; // Lighter gray for grid lines
    const OBSTACLE_COLOR: [u8; 3] = [150, 0, 0]; // Red (for obstacles)
    const CENTER_COLOR: [u8; 3] = [0, 0, 0]; // Black (for center points)
    const GREEN_SHADES: [[u8; 3]; 21] = [
        [240, 255, 240], // Honeydew (very light green)
        [220, 255, 220],
        [200, 255, 200],
        [180, 255, 180],
        [160, 255, 160],
        [140, 255, 140],
        [120, 255, 120],
        [100, 255, 100],
        [80, 220, 80],
        [60, 200, 60],
        [40, 180, 40],
        [30, 160, 30],
        [20, 140, 20],
        [15, 120, 15],
        [10, 100, 10],
        [8, 80, 8],
        [6, 60, 6],
        [4, 40, 4],
        [2, 20, 2],
        [0, 64, 0],
        [0, 44, 0], // Pure dark green
    ];

    // Fill the image with grid color first
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb(GRID_BACKGROUND_COLOR);
    }

    // Draw colored cells for covered areas
    for y in 0..model.grid_cells_y {
        // Convert grid y to image y (invert y axis to match terminal output)
        let img_y = model.grid_cells_y - 1 - y;

        for x in 0..model.grid_cells_x {
            // Fill the cell with color (using the uniform cell size)
            let start_x = x as u32 * cell_size;
            let start_y = img_y as u32 * cell_size;
            let cell = &model.grid.as_ref().unwrap().cells[x][y];

            let color = match cell {
                Cell::Obstacle => Some(OBSTACLE_COLOR),
                Cell::Empty => None,
                Cell::Covered(info) => {
                    let color_idx = info.times_visited.min(GREEN_SHADES.len() - 1);
                    Some(GREEN_SHADES[color_idx])
                }
                Cell::CenterPoint(_) if model.track_center => Some(CENTER_COLOR),
                _ => None,
            };

            if let Some(color) = color {
                for cy in start_y..start_y + cell_size {
                    for cx in start_x..start_x + cell_size {
                        if cx < img_width && cy < img_height {
                            img.put_pixel(cx, cy, image::Rgb(color));
                        }
                    }
                }
            }
        }
    }

    if model.show_gridlines {
        // Draw vertical grid lines for each work coordinates
        for x in 0..model.grid_width.round() as u32 {
            let x_pos = model.grid.as_ref().unwrap().coordinate_to_grid_x(x as f64);
            for y in 0..img_height {
                img.put_pixel(x_pos as u32 * cell_size, y, image::Rgb(GRID_LINE_COLOR));
            }
        }

        // Draw horizontal grid lines for each work coordinates
        for y in 0..model.grid_height.round() as u32 {
            let y_pos = model.grid.as_ref().unwrap().coordinate_to_grid_y(y as f64);
            for x in 0..img_width {
                img.put_pixel(x, y_pos as u32 * cell_size, image::Rgb(GRID_LINE_COLOR));
            }
        }
    }

    if let Some(filename) = override_filename {
        img.save(filename)?;
    } else {
        img.save(model.image_file_name.as_ref().unwrap())?;
    }

    Ok(())
}
