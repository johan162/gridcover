use std::cmp::min;

use crate::color_theme::{ColorTheme, ColorThemeManager};
use crate::model::SimModel;
use crate::model::grid::Cell;
use ab_glyph::{FontArc, PxScale};
use colored::Colorize;
use imageproc::drawing::draw_text_mut;

mod font_dejavusans;
mod font_dejavusansbold;

// use crate::image::font_dejavusans::DEJAVUSANS;
use crate::image::font_dejavusansbold::DEJAVUSANS_BOLD;

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

pub fn draw_text(img: &mut image::RgbImage, text: &str, x: usize, y: usize, color: &[u8; 3]) {
    // Load the font from the embedded font data
    let font = FontArc::try_from_slice(DEJAVUSANS_BOLD).expect("Error loading font data");
    let scale = PxScale::from(50.0);
    draw_text_mut(
        img,
        image::Rgb(*color),
        x as i32,
        y as i32,
        scale,
        &font,
        text,
    );
}

/// Draw a filled rectangle with color brightness adjusted with given factor
/// A color factor > 1.0 will make the color brighter, < 1.0 will dim it
pub fn draw_filled_rect(
    img: &mut image::RgbImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color_factor: f32,
) {
    for i in 0..height {
        for j in 0..width {
            let pixel = img.get_pixel(x + j, y + i);
            let dimmed_pixel = image::Rgb([
                (pixel[0] as f32 * color_factor) as u8,
                (pixel[1] as f32 * color_factor) as u8,
                (pixel[2] as f32 * color_factor) as u8,
            ]);
            img.put_pixel(x + j, y + i, dimmed_pixel);
        }
    }
}

#[allow(dead_code)]
pub fn vert_line_to_image(
    img: &mut image::RgbImage,
    x: u32,
    start_y: u32,
    end_y: u32,
    color: image::Rgb<u8>,
) {
    for y in start_y..=end_y {
        img.put_pixel(x, y, color);
    }
}

#[allow(dead_code)]
pub fn horiz_line_to_image(
    img: &mut image::RgbImage,
    y: u32,
    start_x: u32,
    end_x: u32,
    color: image::Rgb<u8>,
) {
    for x in start_x..=end_x {
        img.put_pixel(x, y, color);
    }
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
        if model.verbosity > 3 {
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
        if model.verbosity > 3 {
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
    let pixels_cell_size = std::cmp::min(
        base_img_width_pixels / model.grid_cells_x as u32,
        base_img_height_pixels / model.grid_cells_y as u32,
    );

    // Recalculate image dimensions using the uniform cell size
    let img_width = pixels_cell_size * model.grid_cells_x as u32;
    let img_height = pixels_cell_size * model.grid_cells_y as u32;

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
            let start_x = x as u32 * pixels_cell_size;
            let start_y = img_y as u32 * pixels_cell_size;
            let cell = &model.grid.as_ref().unwrap().cells[x][y];

            let color = match cell {
                Cell::Obstacle => Some(theme.obstacle_color),
                Cell::Empty => None,
                Cell::Covered(info) => Some(theme.get_coverage_color(info.times_visited)),
                Cell::CenterPoint(_) if model.track_center => Some(theme.center_color),
                _ => None,
            };

            if let Some(color) = color {
                for cy in start_y..start_y + pixels_cell_size {
                    for cx in start_x..start_x + pixels_cell_size {
                        if cx < img_width && cy < img_height {
                            img.put_pixel(cx, cy, image::Rgb(color));
                        }
                    }
                }
            }
        }
    }

    if model.show_gridlines {
        draw_grid_lines(&mut img, model, pixels_cell_size, theme.grid_line_color);
    }

    let show_quad_tree = model.show_quad_tree && model.grid.as_ref().unwrap().quadtree.is_some();
    if show_quad_tree {
        draw_quad_tree(&mut img, model, pixels_cell_size);
    }

    if model.show_image_label {
        add_time_and_coverage_to_image(&mut img, model)?;
    }

    Ok(img)
}

pub fn add_time_and_coverage_to_image(
    img: &mut image::RgbImage,
    model: &crate::model::SimModel,
) -> Result<(), Box<dyn std::error::Error>> {
    let theme_manager = ColorThemeManager::new();
    let theme = theme_manager.get_theme(model.color_theme.as_deref().unwrap_or("default"));

    draw_filled_rect(img, 0, 0, 230, 100, theme.text_background_adjustment);

    let total_seconds = model.sim_time_elapsed as u64;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let sim_time = format!("{hours:02}:{minutes:02}:{seconds:02}");
    let coverage_percent = model.grid.as_ref().unwrap().get_coverage_percent();
    let coverage_text = format!("{coverage_percent:.1}%",);

    let (x, mut y) = (5, 3);
    draw_text(img, &sim_time, x, y, &theme.text_color);
    y += 45;
    draw_text(img, &coverage_text, x, y, &theme.text_color);

    Ok(())
}

fn draw_grid_lines(
    img: &mut image::RgbImage,
    model: &crate::model::SimModel,
    pixels_cell_size: u32,
    grid_color: [u8; 3],
) {
    let grid = model.grid.as_ref().unwrap();

    for x in 0..model.grid_width.round() as u32 {
        let x_pos = grid.world_coordinate_to_grid_x(x as f64);
        for y in 0..img.height() {
            img.put_pixel(x_pos as u32 * pixels_cell_size, y, image::Rgb(grid_color));
        }
    }

    // Draw horizontal grid lines for each world coordinates
    for y in 0..model.grid_height.round() as u32 {
        let y_pos = grid.world_coordinate_to_grid_y(y as f64);
        for x in 0..img.width() {
            img.put_pixel(x, y_pos as u32 * pixels_cell_size, image::Rgb(grid_color));
        }
    }
}

fn draw_quad_tree(
    img: &mut image::RgbImage,
    model: &crate::model::SimModel,
    pixels_cell_size: u32,
) {
    let tree = model.grid.as_ref().unwrap().quadtree.as_ref().unwrap();
    let grid = model.grid.as_ref().unwrap();
    let grid_cells_y = model.grid_cells_y;
    // Draw the quad-tree structure on the image
    if let Some(children) = &tree.root.children {
        for child in children.iter() {
            draw_quad_tree_nodes(img, child, grid, pixels_cell_size, grid_cells_y);
        }
    }
}

fn draw_quad_tree_nodes(
    img: &mut image::RgbImage,
    node: &crate::model::quadtree::QuadTreeNode,
    grid: &crate::model::grid::Grid,
    pixels_cell_size: u32,
    grid_cells_y: usize,
) {
    let quad_tree_color = image::Rgb([200, 0, 0]); // Red color for quad-tree bounds

    // Draw the bounds of the quad-tree node
    let (start_x, start_y) = grid.world_coordinate_to_grid(node.bounds.x, node.bounds.y);
    let (end_x, end_y) = grid.world_coordinate_to_grid(
        node.bounds.x + node.bounds.width,
        node.bounds.y + node.bounds.height,
    );

    let index_x_start = min(start_x as u32 * pixels_cell_size, img.width() - 1);
    let index_x_end = min(end_x as u32 * pixels_cell_size, img.width() - 1);

    let top_line_y = min(
        (grid_cells_y - start_y) as u32 * pixels_cell_size,
        img.height() - 1,
    );
    let bottom_line_y = min(
        (grid_cells_y - end_y) as u32 * pixels_cell_size,
        img.height() - 1,
    );

    // We swap y coordinates because image coordinates start from top-left and world coordinates start from bottom-left
    let index_y_end = top_line_y;
    let index_y_start = bottom_line_y;

    // If the node have no obstacles then make the node area within the boundries dimmed
    if !node.has_obstacle && node.is_leaf {
        // Read the pixel and then make it 30% darker
        for y in index_y_start..=index_y_end {
            for x in index_x_start..=index_x_end {
                let pixel = img.get_pixel(x, y);
                let dimmed_pixel = image::Rgb([
                    (pixel[0] as f32 * 0.7) as u8,
                    (pixel[1] as f32 * 0.7) as u8,
                    (pixel[2] as f32 * 0.7) as u8,
                ]);
                img.put_pixel(x, y, dimmed_pixel);
            }
        }
    }

    // Draw the four lines in the quad-tree node with double thickness
    for x in index_x_start..=index_x_end {
        img.put_pixel(x, top_line_y, quad_tree_color);
        img.put_pixel(x, min(top_line_y + 1, img.height() - 1), quad_tree_color);
    }

    // Draw bottom line
    for x in index_x_start..=index_x_end {
        img.put_pixel(x, bottom_line_y, quad_tree_color);
        if bottom_line_y > 0 {
            img.put_pixel(x, bottom_line_y - 1, quad_tree_color);
        }
    }

    // Draw left line
    for y in index_y_start..=index_y_end {
        img.put_pixel(index_x_start, y, quad_tree_color);
        img.put_pixel(min(index_x_start + 1, img.width() - 1), y, quad_tree_color);
    }
    // Draw right line
    for y in index_y_start..=index_y_end {
        img.put_pixel(index_x_end, y, quad_tree_color);
        if index_x_end > 0 {
            img.put_pixel(index_x_end - 1, y, quad_tree_color);
        }
    }

    if let Some(children) = &node.children {
        for child in children.iter() {
            draw_quad_tree_nodes(img, child, grid, pixels_cell_size, grid_cells_y);
        }
    }
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
