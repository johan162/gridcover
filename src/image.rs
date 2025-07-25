use crate::color_theme::{ColorTheme, ColorThemeManager};
use crate::model::SimModel;
use crate::model::grid::Cell;
use colored::Colorize;

#[allow(clippy::collapsible_if)]
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
    // Get the color theme
    let theme_manager = ColorThemeManager::new();
    let theme = theme_manager.get_theme(model.color_theme.as_deref().unwrap_or("default"));

    save_grid_image_with_theme(model, override_filename, theme)
}

/// Create a PNG image of the coverage grid with colored squares using a specific theme
fn save_grid_image_with_theme(
    model: &crate::model::SimModel,
    override_filename: Option<String>,
    theme: &ColorTheme,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = create_grid_image_in_memory_with_theme(model, theme)?;
    if let Some(filename) = override_filename {
        img.save(filename)?;
    } else {
        img.save(model.image_file_name.as_ref().unwrap())?;
    }

    Ok(())
}

/// Create an in-memory RGB image of the coverage grid
#[allow(dead_code)]
pub fn create_grid_image_in_memory(
    model: &SimModel,
) -> Result<image::RgbImage, Box<dyn std::error::Error>> {
    let theme_manager = ColorThemeManager::new();
    let theme = theme_manager.get_theme(model.color_theme.as_deref().unwrap_or("default"));

    create_grid_image_in_memory_with_theme(model, theme)
}

/// Create a PNG image of the coverage grid with colored squares using a specific theme
fn create_grid_image_in_memory_with_theme(
    model: &crate::model::SimModel,
    theme: &ColorTheme,
) -> Result<image::RgbImage, Box<dyn std::error::Error>> {
    // Convert mm to pixels using DPI (Dots Per Inch)
    let pixels_per_mm = model.dpi as f64 / 25.4;

    let mut base_img_width_pixels = (model.image_width_mm as f64 * pixels_per_mm).round() as u32;
    let mut base_img_height_pixels = (model.image_height_mm as f64 * pixels_per_mm).round() as u32;

    if base_img_height_pixels < model.grid_cells_y as u32 {
        if model.verbosity > 1 {
            eprintln!(
                "{} {} mm",
                "Notice. Adjusting image height to fit grid height. New height at given DPI:"
                    .yellow()
                    .bold(),
                (model.grid_cells_y as u32 + 1) / pixels_per_mm as u32
            );
        }
        base_img_height_pixels = model.grid_cells_y as u32 + 1;
    }

    if base_img_width_pixels < model.grid_cells_x as u32 {
        if model.verbosity > 1 {
            eprintln!(
                "{} {} mm",
                "Notice.! Adjusting image width to fit grid width. New width at given DPI:"
                    .yellow()
                    .bold(),
                (model.grid_cells_x as u32 + 1) / pixels_per_mm as u32
            );
        }
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

    // Fill the image with grid color first
    for pixel in img.pixels_mut() {
        *pixel = image::Rgb(theme.grid_background_color);
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
                Cell::Obstacle => Some(theme.obstacle_color),
                Cell::Empty => None,
                Cell::Covered(info) => Some(theme.get_coverage_color(info.times_visited)),
                Cell::CenterPoint(_) if model.track_center => Some(theme.center_color),
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
                img.put_pixel(
                    x_pos as u32 * cell_size,
                    y,
                    image::Rgb(theme.grid_line_color),
                );
            }
        }

        // Draw horizontal grid lines for each work coordinates
        for y in 0..model.grid_height.round() as u32 {
            let y_pos = model.grid.as_ref().unwrap().coordinate_to_grid_y(y as f64);
            for x in 0..img_width {
                img.put_pixel(
                    x,
                    y_pos as u32 * cell_size,
                    image::Rgb(theme.grid_line_color),
                );
            }
        }
    }

    Ok(img)
}

/// Get available color theme names
#[allow(dead_code)]
pub fn get_available_themes() -> Vec<String> {
    let theme_manager = ColorThemeManager::new();
    theme_manager
        .list_theme_names()
        .into_iter()
        .cloned()
        .collect()
}

/// Create a color theme manager for external use
#[allow(dead_code)]
pub fn create_theme_manager() -> ColorThemeManager {
    ColorThemeManager::new()
}
