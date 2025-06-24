use crate::args;
use crate::model::{SimModel, grid::Grid, setup_grid_size};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;

mod circle;
mod line;
mod polygon;
mod rectangle;

#[derive(Debug, Deserialize, Serialize)]
pub struct MapFile {
    pub name: String,
    pub description: Option<String>,
    pub grid: Option<GridConfig>,
    pub obstacles: Vec<ObstacleType>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GridConfig {
    pub width: Option<f64>,
    pub height: Option<f64>,
    // pub cell_size: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum ObstacleType {
    #[serde(rename = "rectangle")]
    Rectangle {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        name: Option<String>,
    },

    #[serde(rename = "circle")]
    Circle {
        x: f64,
        y: f64,
        radius: f64,
        name: Option<String>,
    },

    #[serde(rename = "polygon")]
    Polygon {
        points: Vec<[f64; 2]>,
        name: Option<String>,
    },

    #[serde(rename = "line")]
    Line {
        points: Vec<[f64; 2]>,
        width: f64,
        name: Option<String>,
    },
}

fn load_map_file<P: AsRef<Path>>(path: P) -> Result<MapFile, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let map_file: MapFile = serde_yaml::from_str(&content)?;
    Ok(map_file)
}

fn apply_obstacles_to_grid(grid: &mut Grid, map: &MapFile) {
    for obstacle in &map.obstacles {
        match obstacle {
            ObstacleType::Rectangle {
                x,
                y,
                width,
                height,
                ..
            } => {
                rectangle::apply_rectangle_obstacle(grid, *x, *y, *width, *height);
            }
            ObstacleType::Circle { x, y, radius, .. } => {
                circle::apply_circle_obstacle(grid, *x, *y, *radius);
            }
            ObstacleType::Polygon { points, .. } => {
                polygon::apply_polygon_obstacle(grid, points);
            }
            ObstacleType::Line { points, width, .. } => {
                line::apply_line_obstacle(grid, points.as_slice(), *width);
            }
        }
    }
}

pub fn load_optional_mapfile(args: &args::Args, model: &mut SimModel) {
    // Load map file if specified
    if let Some(map_path) = args.map_file_name.clone() {
        match load_map_file(&map_path) {
            Ok(map) => {
                // Apply grid overrides if specified in map
                if let Some(grid_config) = &map.grid {
                    if let Some(width) = grid_config.width {
                        model.grid_width = width;
                    }
                    if let Some(height) = grid_config.height {
                        model.grid_height = height;
                    }
                    // if let Some(cell_size) = grid_config.cell_size {
                    //     model.cell_size = cell_size;
                    // }
                }
                if let Err(e) = setup_grid_size(model) {
                    eprintln!(
                        "{}",
                        format!("Failed to setup grid size after map loading: {e}")
                            .color(colored::Color::Red)
                            .bold()
                    );
                    std::process::exit(1);
                }
                model.map_file = Some(map);
            }
            Err(e) => {
                eprintln!("Failed to load map file: {e}");
                std::process::exit(1);
            }
        }
    }
}

pub fn try_apply_mapfile_to_model(model: &mut SimModel) {
    // Load map file if specified
    if let Some(map) = &model.map_file {
        apply_obstacles_to_grid(model.grid.as_mut().unwrap(), map);
        model.grid.as_mut().unwrap().update_obstacle_count();
        model.grid_cells_obstacles_count = model.grid.as_ref().unwrap().get_obstacle_count();
        model.num_obstacles = map.obstacles.len();
    }
}
