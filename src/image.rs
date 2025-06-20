use colored::Colorize;

/// Create a PNG image of the coverage grid with colored squares
///
/// # Arguments
///
/// * `width` - Width of the grid in cells
/// * `height` - Height of the grid in cells
/// * `coverage_grid` - The grid containing coverage information
/// * `image_width_mm` - The desired width of the image in millimeters
/// * `image_height_mm` - The desired height of the image in millimeters
/// * `output_path` - Path where to save the PNG image
pub fn save_grid_image(model: &crate::model::SimModel) -> Result<(), Box<dyn std::error::Error>> {
    // Convert mm to pixels (300 DPI for quality monitors)
    // 300 pixels per inch, 25.4 mm per inch is what we get on quality monitors
    let pixels_per_mm = model.dpi as f64 / 25.4;
    // println!("Pixels per mm: {pixels_per_mm}");
    // Calculate base dimensions
    let mut base_img_width = (model.image_width_mm as f64 * pixels_per_mm).round() as u32;
    let mut base_img_height = (model.image_height_mm as f64 * pixels_per_mm).round() as u32;
    // println!("Base image size: {base_img_width}x{base_img_height}");

    if base_img_height < model.grid_cells_y as u32 {
        eprintln!(
            "{} {} mm",
            "Warning! Adjusting image height to fit grid height. New height at given DPI:"
                .yellow()
                .bold(),
            (model.grid_cells_y as u32 + 1) / pixels_per_mm as u32
        );
        base_img_height = model.grid_cells_y as u32 + 1;
    }

    if base_img_width < model.grid_cells_x as u32 {
        eprintln!(
            "{} {} mm",
            "Warning! Adjusting image width to fit grid. New width at given DPI:"
                .yellow()
                .bold(),
            (model.grid_cells_x as u32 + 1) / pixels_per_mm as u32
        );
        base_img_width = model.grid_cells_x as u32 + 1;
    }

    // Calculate cell size to ensure perfect squares
    // Take the smaller dimension to make sure image fits within requested size
    let cell_size = std::cmp::min(
        base_img_width / model.grid_cells_x as u32,
        base_img_height / model.grid_cells_y as u32,
    );
    // println!("Cell size: {cell_size}");

    // Recalculate image dimensions using the uniform cell size
    let img_width = cell_size * model.grid_cells_x as u32;
    let img_height = cell_size * model.grid_cells_y as u32;
    // println!("Image size: {img_width}x{img_height}");
    // Create a new RGB image buffer
    let mut img = image::RgbImage::new(img_width, img_height);

    // Define colors used in grid``
    const GRID_COLOR: [u8; 3] = [150, 150, 150]; // Dark gray
    const CENTER_COLOR: [u8; 3] = [255, 255, 255]; // White (for center points)
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
        *pixel = image::Rgb(GRID_COLOR);
    }

    // Draw colored cells for covered areas
    for (y, row) in model.coverage_grid.iter().enumerate() {
        // Convert grid y to image y (invert y axis to match terminal output)
        let img_y = model.grid_cells_y - 1 - y;

        for (x, info) in row.iter().enumerate() {
            if info.covered {
                let color = if model.track_center
                    && info.times_visited == crate::cells::CENTERPOINT_MAGIC_CONSTANT
                {
                    CENTER_COLOR
                } else {
                    //let color_idx = info.bounce_number.min(colors.len() - 1);
                    let color_idx = info.times_visited.min(GREEN_SHADES.len() - 1);
                    GREEN_SHADES[color_idx]
                };

                // Fill the cell with color (using the uniform cell size)
                let start_x = x as u32 * cell_size;
                let start_y = img_y as u32 * cell_size;

                // Draw the filled cell with a small border
                let border = 0; // 1 pixel border
                for cy in start_y + border..start_y + cell_size - border {
                    for cx in start_x + border..start_x + cell_size - border {
                        if cx < img_width && cy < img_height {
                            img.put_pixel(cx, cy, image::Rgb(color));
                        }
                    }
                }
            }
        }
    }

    img.save(model.image_file_name.as_ref().unwrap())?;

    Ok(())
}
