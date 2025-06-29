use std::fs;

use crate::model::grid::Grid;
use crate::{args, mapfile};
use chrono::Duration;
use colored::Colorize;
use rand::Rng;
use serde_json::json;

const MIN_RADIUS: f64 = 0.01;
const MIN_BLADE_LEN: f64 = 0.01;

pub mod boundingbox;
pub mod coverageinfo;
pub mod cuttertype;
pub mod grid;
pub mod papersize;

#[derive(Debug)]
pub struct SimModel {
    pub start_x: f64,
    pub start_y: f64,
    pub start_dir_x: f64,
    pub start_dir_y: f64,
    pub start_angle_deg: f64,
    pub step_size: f64,
    pub radius: f64,
    pub grid_cells_x: usize,
    pub grid_cells_y: usize,
    pub grid_width: f64,
    pub grid_height: f64,
    pub cell_size: f64,
    pub velocity: f64,
    pub sim_time_elapsed: f64,
    pub segment_number: usize,
    pub coverage_percent: f64,
    pub coverage_count: usize,
    pub grid_cells_obstacles_count: usize,
    pub num_obstacles: usize,
    pub max_visited_number: usize,
    pub min_visited_number: usize,
    pub perturb: bool,
    pub cpu_time: Duration,
    pub sim_steps: u64,
    pub distance_covered: f64,
    pub stop_coverage: f64,
    pub stop_time: f64,
    pub stop_bounces: usize,
    pub stop_simsteps: u64,
    pub stop_distance: f64,
    pub parallel: bool,
    pub image_width_mm: u32,
    pub image_height_mm: u32,
    pub image_file_name: Option<String>,
    pub map_file_name: Option<String>,
    pub show_gridlines: bool,
    pub verbosity: usize,
    pub track_center: bool,
    pub show_progress: bool,
    pub blade_len: f64,
    pub cutter_type: cuttertype::CutterType,
    pub dpi: u32,
    pub perturb_segment: bool,
    pub perturb_segment_percent: f64,
    pub grid: Option<Grid>,
    pub bb: boundingbox::BoundingBox,
    pub battery_run_time: f64,
    pub battery_charge_time: f64,
    pub battery_charge_count: usize,
    pub battery_charge_left: f64,
    pub paper_size: papersize::PaperSize,
    pub map_file: Option<mapfile::MapFile>,
    pub quiet: bool,
    pub generate_frames: bool,
    pub frames_dir: String,
    pub frame_rate: u64,
    pub steps_per_frame: u64,
    pub create_animation: bool,
    pub animation_file_name: String,
    pub hw_encoding: bool,
    pub delete_frames: bool,
}

// Define a constant for the simulation step size factor
// This factor determines how many simulation steps are taken per square (cell) size
// To get a full coverage of squares we should always have a step size that is a fraction of the square size.
const SIMULATION_STEP_SIZE_FRACTION_OF_CELL: f64 = 3.0 / 5.0;

impl SimModel {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        start_x: f64,
        start_y: f64,
        start_dir_x: f64,
        start_dir_y: f64,
        start_angle_deg: f64,
        step_size: f64,
        radius: f64,
        grid_width: f64,
        grid_height: f64,
        cell_size: f64,
        velocity: f64,
        stop_coverage: f64,
        stop_time: f64,
        stop_bounces: usize,
        stop_simsteps: u64,
        stop_distance: f64,
        parallel: bool,
        image_width_mm: u32,
        image_height_mm: u32,
        image_file_name: Option<String>,
        show_gridlines: bool,
        verbosity: usize,
        track_center: bool,
        show_progress: bool,
        blade_len: f64,
        cutter_type: cuttertype::CutterType,
        dpi: u32,
        perturb_segment: bool,
        perturb_segment_percent: f64,
        battery_run_time: f64,
        battery_charge_time: f64,
        paper_size: papersize::PaperSize,
        map_file_name: Option<String>,
        quiet: bool,
        generate_frames: bool,
        frames_dir: String,
        frame_rate: u64,
        create_animation: bool,
        animation_file_name: String,
        hw_encoding: bool,
        delete_frames: bool,
    ) -> Self {
        Self {
            start_x,
            start_y,
            start_dir_x,
            start_dir_y,
            start_angle_deg,
            step_size,
            radius,
            grid_cells_x: 0,
            grid_cells_y: 0,
            grid_width,
            grid_height,
            cell_size,
            velocity,
            sim_time_elapsed: 0.0,
            segment_number: 0,
            coverage_percent: 0.0,
            coverage_count: 0,
            max_visited_number: 0,
            min_visited_number: 0,
            grid_cells_obstacles_count: 0,
            perturb: true,
            cpu_time: Duration::zero(),
            sim_steps: 0,
            distance_covered: 0.0,
            stop_coverage,
            stop_time,
            stop_bounces,
            stop_simsteps,
            stop_distance,
            parallel,
            image_width_mm,
            image_height_mm,
            image_file_name,
            map_file_name,
            show_gridlines,
            verbosity,
            track_center,
            show_progress,
            blade_len,
            cutter_type,
            dpi,
            perturb_segment,
            perturb_segment_percent,
            grid: None, // Will be initialized later
            bb: boundingbox::BoundingBox::init(grid_width, grid_height, radius), // TODO: This might not be set at this staeg!
            battery_run_time,
            battery_charge_time,
            battery_charge_count: 0,
            battery_charge_left: 100.0,
            paper_size,
            num_obstacles: 0,
            map_file: None,
            quiet,
            generate_frames,
            frames_dir,
            frame_rate,
            steps_per_frame: 1,
            create_animation,
            animation_file_name,
            hw_encoding,
            delete_frames,
        }
    }

    pub fn init(args: &crate::args::Args) -> Self {
        Self::new(
            args.start_x,
            args.start_y,
            args.dir_x,
            args.dir_y,
            0.0,
            args.step_size,
            args.radius,
            args.grid_width,
            args.grid_height,
            args.square_size,
            args.velocity,
            args.stop_coverage,
            args.stop_time,
            args.stop_bounces,
            args.stop_simsteps,
            args.stop_distance,
            args.parallel,
            args.image_width_mm,
            args.image_height_mm,
            args.image_file_name.clone(),
            args.show_gridlines,
            args.verbosity,
            args.track_center,
            args.show_progress,
            args.blade_len,
            args.cutter_type,
            args.dpi,
            args.perturb_segment,
            args.perturb_segment_percent / 100.0,
            args.battery_run_time,
            args.battery_charge_time,
            args.paper_size,
            args.map_file_name.clone(),
            args.quiet,
            args.generate_frames,
            args.frames_dir.clone(),
            args.frame_rate,
            args.create_animation,
            args.animation_file_name.clone(),
            args.hw_encoding,
            args.delete_frames,
        )
    }

    /// Get all model parameters as a formatted JSON string
    pub fn get_model_as_json(&self) -> serde_json::Value {
        let json = json!({
            "Model": {
                "Cutter": {
                    "Type": self.cutter_type.as_str(),
                    "Blade Length": self.blade_len,
                    "Radius": self.radius,
                    "Battery": {
                        "Run Time": self.battery_run_time,
                        "Charge Time": self.battery_charge_time,
                    },
                    "Velocity": self.velocity,
                },
                "Simulation": {
                    "Verbosity": self.verbosity,
                    "Quiet": self.quiet,
                    "Show Progress": self.show_progress,
                    "Track Center": self.track_center,
                    "Step Size": self.step_size,
                    "Perturb at Bounces": self.perturb,
                    "Perturb Segment": self.perturb_segment,
                    "Perturb Segment Percent": self.perturb_segment_percent * 100.0,
                },
                "Frames": {
                    "Enabled": self.generate_frames,
                    "Directory": self.frames_dir,
                    "Rate (fps)": self.frame_rate,
                    "Create Animation": self.create_animation,
                    "Animation File Name": self.animation_file_name,
                    "HW Encoding": self.hw_encoding,
                    "Delete Frames": self.delete_frames,
                },
                "Start": {
                    "Position": {
                        "X": self.start_x,
                        "Y": self.start_y,
                    },
                    "Direction": {
                        "DirX": self.start_dir_x,
                        "DirY": self.start_dir_y,
                        "Angle (deg)": self.start_angle_deg,
                    }
                },
                "Grid": {
                    "Hor. Cells": self.grid_cells_x,
                    "Ver. Cells": self.grid_cells_y,
                    "Total Cells": self.grid_cells_x * self.grid_cells_y,
                    "Width (units)": self.grid_width,
                    "Height (units)": self.grid_height,
                    "Cell Size": self.cell_size,
                    "Map File Name": self.map_file_name.as_ref().unwrap_or(&"None".to_string()),
                    "Obstacles": {
                        "Num obstacles": self.num_obstacles,
                        "Cells with obstacle": self.grid_cells_obstacles_count,
                        "Percent": (self.grid_cells_obstacles_count as f64
                            / (self.grid_cells_x * self.grid_cells_y) as f64)
                            * 100.0,
                    },
                },
                "Image" : {
                    "Image Size (mm)": {
                        "Width": self.image_width_mm,
                        "Height": self.image_height_mm,
                    },
                    "Image File Name": self.image_file_name.as_ref().unwrap_or(&"None".to_string()),
                    "DPI": self.dpi,
                    "Paper Size": self.paper_size.get_json(),
                    "Show Gridlines": self.show_gridlines,
                },
                "Stop Conditions": {
                    "Bounces": self.stop_bounces,
                    "Time (seconds)": self.stop_time,
                    "Coverage (%)": self.stop_coverage,
                    "Simulation Steps": self.stop_simsteps,
                    "Distance (units)": self.stop_distance,
                },
            }
        });
        json
    }

    pub fn get_theorethical_minimum_cutting_time(&self) -> (usize, usize, usize, f64) {
        let mut theoretical_minimum_time_seconds = 0;
        if self.stop_coverage > 0.0 {
            // Theoretical minimum time is calculated based on the grid size, radius, and velocity
            theoretical_minimum_time_seconds =
                self.grid_cells_x * self.grid_cells_y - self.grid_cells_obstacles_count;

            theoretical_minimum_time_seconds /=
                (2.0 * self.radius / self.cell_size).floor() as usize;

            theoretical_minimum_time_seconds /= (self.velocity / self.cell_size).floor() as usize;

            theoretical_minimum_time_seconds =
                (theoretical_minimum_time_seconds as f64 * self.coverage_percent / 100.0) as usize;
        }

        let t_hours = theoretical_minimum_time_seconds / 3600;
        let t_minutes = (theoretical_minimum_time_seconds % 3600) / 60;
        let t_seconds = theoretical_minimum_time_seconds % 60;

        // Efficiency is calculated as the ratio of the theoretical minimum time to the actual simulation time
        // expressed as a percentage. It is only meaningful if the simulation was stopped by coverage condition.
        let efficiency = if self.stop_coverage > 0.0
            && self.coverage_percent >= self.stop_coverage
            && self.sim_time_elapsed > 0.0
        {
            (theoretical_minimum_time_seconds as f64 / self.sim_time_elapsed) * 100.0
        } else {
            0.0
        };

        (t_hours, t_minutes, t_seconds, efficiency)
    }

    /// A version of print_simulation_results() that outputs results in JSON format
    pub fn get_simulation_result_as_json(&self) -> serde_json::Value {
        let total_seconds = self.sim_time_elapsed as u64;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        let (t_hours, t_minutes, t_seconds, efficiency) =
            self.get_theorethical_minimum_cutting_time();

        let json = json!({
            "Simulation Result": {
                "Coverage": {
                    "Percent": self.coverage_percent,
                    "Cells": self.coverage_count,
                    "Bounces": self.segment_number,
                    "Max visited": self.max_visited_number,
                    "Min visited": self.min_visited_number,
                },
                "Cutter": {
                    "Type": self.cutter_type.as_str(),
                    "Blade Length": self.blade_len,
                    "Radius": self.radius,
                    "Velocity": self.velocity,
                    "Distance": self.distance_covered,
                    "Cells under": (2.0*self.radius/self.cell_size).floor() * (2.0*self.radius/self.cell_size).floor(),
                    "Battery": {
                        "Run time": self.battery_run_time,
                        "Charge time": self.battery_charge_time,
                        "Charge count": self.battery_charge_count,
                        "Charge left (%)": self.battery_charge_left,
                    }
                },
                "Time": {
                     "CPU time": format!("{:02}:{:02}:{:02}.{:03}",
                        self.cpu_time.num_hours(),
                        self.cpu_time.num_minutes(),
                        self.cpu_time.num_seconds() % 60,
                        self.cpu_time.num_milliseconds() % 1000),

                    "Cutting time": format!("{:02}:{:02}:{:02}",
                        hours, minutes, seconds),
                    "Min.Cov.Time": format!("{:02}:{:02}:{:02}",
                        t_hours, t_minutes, t_seconds),
                    "Efficiency": format!("{efficiency:.2}").parse::<f64>().unwrap(),
                },
                "Start": {
                    "Position": {
                        "X": self.start_x,
                        "Y": self.start_y,
                    },
                    "Direction": {
                        "X": self.start_dir_x,
                        "Y": self.start_dir_y,
                    },
                    "Angle (degrees)": self.start_angle_deg,
                },
                "Grid": {
                    "Hor.Cells": self.grid_cells_x,
                    "Vert.Cells": self.grid_cells_y,
                    "Total cells": self.grid_cells_x * self.grid_cells_y,
                    "Obstacles": {
                        "NumCells": self.grid_cells_obstacles_count,
                        "Percent": (self.grid_cells_obstacles_count as f64
                            / (self.grid_cells_x * self.grid_cells_y) as f64)
                            * 100.0,
                    },
                    "Cell side (units)": self.cell_size,
                    "Width (units)": self.grid_width,
                    "Height (units)": self.grid_height,
                },
                "Steps": {
                    "Total #": self.sim_steps,
                    "Length (units)": self.step_size,
                    "Steps/cell": self.step_size / self.cell_size,
                    "Seconds/step": self.step_size / self.velocity,
                    "Steps/second": (self.velocity / self.step_size).floor() as u32,
                },
                "Frames": {
                    "Enabled": self.generate_frames,
                    "Directory": self.frames_dir,
                    "Rate (fps)": self.frame_rate,
                    "Steps per frame": self.steps_per_frame,
                    "Animation": self.create_animation,
                    "Animation file name": self.animation_file_name,
                },
                "Output image": {
                    "Paper size": self.paper_size.get_json(),
                    "Show gridlines": self.show_gridlines,
                    "File name": if self.image_file_name.is_none() {
                        "".to_string()
                    } else {
                        self.image_file_name.as_ref().unwrap().clone()
                    },
                    "DPI": self.dpi,
                    "Pixels": {
                        "width": (self.image_width_mm as f64 * self.dpi as f64 / 25.4).round() as u32,
                        "height": (self.image_height_mm as f64 * self.dpi as f64 / 25.4).round() as u32,
                    }
                },
            }
        });

        json
    }

    pub fn get_simulation_result_short_as_json(&self) -> serde_json::Value {
        let (t_hours, t_minutes, t_seconds, efficiency) =
            self.get_theorethical_minimum_cutting_time();

        json!({
            "Simulation Result (Short)": {
                "Coverage": {
                    "Percent": self.coverage_percent,
                    "Bounces": self.segment_number,
                    "Distance": self.distance_covered,
                },
                "Time": {
                    "CPU": format!("{:02}:{:02}:{:02}.{:03}",
                        self.cpu_time.num_hours(),
                        self.cpu_time.num_minutes(),
                        self.cpu_time.num_seconds() % 60,
                        self.cpu_time.num_milliseconds() % 1000),
                    "Cutting": format!("{:02}:{:02}:{:02}",
                        self.sim_time_elapsed as u64 / 3600,
                        (self.sim_time_elapsed as u64 % 3600) / 60,
                        self.sim_time_elapsed as u64 % 60),
                    "Min.Cov.Time": format!("{:02}:{:02}:{:02}",
                        t_hours, t_minutes, t_seconds),
                    "Efficiency": format!("{efficiency:.2}").parse::<f64>().unwrap(),
                },
                "Cutter": {
                    "Type": self.cutter_type.as_str(),
                    "Blade Length": self.blade_len,
                    "Radius": self.radius,
                    "Velocity": self.velocity,
                    "Battery": {
                        "Charge count": self.battery_charge_count,
                        "Charge left (%)": self.battery_charge_left,
                    }
                },
            }
        })
    }

    /// Print the simulation results in JSON format to the console
    pub fn print_simulation_results_as_json(&self) {
        let json = self.get_simulation_result_as_json();
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }

    /// Print the simulation results in JSON format to the console
    pub fn print_simulation_results_short_as_json(&self) {
        let json = self.get_simulation_result_short_as_json();
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }

    #[allow(dead_code)]
    pub fn print_model_as_json(&self) {
        let json = self.get_model_as_json();
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }

    /// Print the simulation results to the console in a human-readable format
    /// by converting the JSON to a label: <value> format.
    pub fn print_simulation_results_txt(&self) {
        let json = self.get_simulation_result_as_json();
        const ROOT_KEY: &str = "Simulation Result";
        println!("{}", ROOT_KEY.color(colored::Color::BrightGreen).bold());
        println!(
            "{}",
            "=".repeat(ROOT_KEY.len())
                .color(colored::Color::BrightGreen)
                .bold()
        );

        json_to_console(&json, ROOT_KEY, 2);
    }

    /// Print the simulation results to the console in a human-readable format
    /// by converting the JSON to a label: <value> format.
    pub fn print_simulation_results_short_txt(&self) {
        let json = self.get_simulation_result_short_as_json();
        const ROOT_KEY: &str = "Simulation Result (Short)";
        println!("{}", ROOT_KEY.color(colored::Color::BrightGreen).bold());
        println!(
            "{}",
            "=".repeat(ROOT_KEY.len())
                .color(colored::Color::BrightGreen)
                .bold()
        );

        json_to_console(&json, ROOT_KEY, 2);
    }

    /// Print the model parameters to the console in a human-readable format
    /// by converting the model JSON to a label: <value> format.
    pub fn print_model_txt(&self) {
        let json = self.get_model_as_json();
        const ROOT_KEY: &str = "Model";
        println!("\n{}", ROOT_KEY.color(colored::Color::BrightGreen).bold());
        println!(
            "{}",
            "=".repeat(ROOT_KEY.len())
                .color(colored::Color::BrightGreen)
                .bold()
        );

        json_to_console(&json, ROOT_KEY, 2);
    }
}

fn json_to_console(json: &serde_json::Value, root_key: &str, indent: usize) {
    let column1 = 40;
    let column2 = column1 - indent;
    let column3 = column2 - indent;
    for (key, value) in json[root_key].as_object().unwrap_or_else(|| {
        let emsg = format!("Internal Error json_to_console(): key '{root_key}' does not exists or is not a valid JSON object").color(colored::Color::Red).bold();
        eprintln!("{emsg}");
        std::process::exit(1);
    }) {
        if let Some(obj) = value.as_object() {
            println!(
                "{:<column1$}",
                key.as_str().bold().color(colored::Color::Green)
            );
            for (sub_key, sub_value) in obj {
                if let Some(sub_obj) = sub_value.as_object() {
                    println!(
                        "{}{:<column2$}",
                        " ".repeat(indent),
                        sub_key.as_str().bold().color(colored::Color::Green)
                    );
                    for (sub_sub_key, sub_sub_value) in sub_obj {
                        println!(
                            "{}{:<column3$}: {}",
                            " ".repeat(indent * 2),
                            format!("{key}.{sub_key}.{sub_sub_key}"),
                            sub_sub_value
                        );
                    }
                } else {
                    println!(
                        "{}{:<column2$}: {}",
                        " ".repeat(indent),
                        format!("{key}.{sub_key}"),
                        sub_value
                    );
                }
            }
        } else {
            // If value is not an object, print it directly
            println!("{key:<column1$}: {value}");
        }
    }
}

fn set_initial_direction(args: &args::Args, rng: &mut impl Rng) -> (f64, f64, f64) {
    let mut current_dir_x = args.dir_x;
    let mut current_dir_y = args.dir_y;
    let angle_deg: f64;

    // If both directions are zero randomize them in range -1 to 1
    if current_dir_x == 0.0 && current_dir_y == 0.0 {
        // Random start angle
        let angle_r = rng.random_range(0.0..std::f64::consts::TAU);
        angle_deg = angle_r.to_degrees();
        current_dir_x = angle_r.cos();
        current_dir_y = angle_r.sin();
    } else {
        // Normalize the direction vector if it's not zero
        // This ensures the direction vector has a length of 1
        let dir_length = (current_dir_x.powi(2) + current_dir_y.powi(2)).sqrt();
        current_dir_x /= dir_length;
        current_dir_y /= dir_length;
        angle_deg = (current_dir_y.atan2(current_dir_x)).to_degrees();
    }

    (current_dir_x, current_dir_y, angle_deg)
}

pub fn init_model(
    args: &args::Args,
    rng: &mut impl Rng,
) -> Result<SimModel, Box<dyn std::error::Error>> {
    // Initialize simulation configuration
    let mut model = SimModel::init(args);

    // Make sure one of the stopping conditions is set
    if args.stop_bounces == 0
        && args.stop_time == 0.0
        && args.stop_coverage == 0.0
        && args.stop_simsteps == 0
        && args.stop_distance == 0.0
    {
        return Err(
            "No stopping condition set (use bounces, sim_time, sim_step, coverage, or distance)."
                .into(),
        );
    }

    // From the geometry we know the following two condions must hold for the simulation to work:
    if model.radius <= MIN_RADIUS {
        return Err(format!("Radius must be greater than {MIN_RADIUS} units").into());
    }

    if model.cell_size <= 0.0 {
        if model.cutter_type == cuttertype::CutterType::Blade {
            if model.blade_len <= MIN_BLADE_LEN || model.blade_len >= model.radius {
                return Err(
                    format!("Blade length must be greater than {MIN_BLADE_LEN} units and less than the radius").into(),
                );
            }
            if model.radius <= model.blade_len * 2.0 {
                return Err(format!(
                    "Radius must be greater than twice the blade length ({}) units",
                    model.blade_len * 2.0
                )
                .into());
            }
            model.cell_size = model.blade_len / 2.5; // Default cell size for blade cutter
        } else {
            model.cell_size = model.radius / 3.0; // Default cell size for circular cutter
        }
    }

    if model.grid_width == 0.0 && model.grid_height == 0.0 {
        // Set default grid size based on cutter radius size
        model.grid_width = model.radius * 50.0;
        model.grid_height = model.radius * 50.0;
    } else if model.grid_width == 0.0 {
        model.grid_width = model.grid_height;
    } else if model.grid_height == 0.0 {
        model.grid_height = model.grid_width;
    }

    if model.grid_width < model.radius * 4.0 || model.grid_height < model.radius * 4.0 {
        return Err(format!(
            "Grid size must be at least 4 times the radius (Minimum {}x{} units)",
            4.0 * model.radius,
            4.0 * model.radius
        )
        .into());
    }

    setup_grid_size(&mut model)?;

    // The size of the simulated grid must be at least three times the radius
    if model.grid_width <= 3.0 * model.radius || model.grid_height <= 3.0 * model.radius {
        return Err(format!(
            "Grid size must be at least three times the radius of the cutter (Minimum {}x{} units)",
            3.0 * model.radius,
            3.0 * model.radius
        )
        .into());
    }

    // If step size is not set then set it to 60% of the cell size
    if model.step_size <= 0.0 {
        model.step_size = model.cell_size * SIMULATION_STEP_SIZE_FRACTION_OF_CELL;
    }

    if model.create_animation {
        model.generate_frames = true;
    }

    // If frame generation is enabled we must adjust step_size so it corresponds to the frame rate

    if model.generate_frames && model.frame_rate > 0 {
        if fs::metadata(&model.frames_dir).is_ok() {
            return Err(format!(
                "Output frame directory '{}' already exists. Please remove it or change the output directory.",
                model.frames_dir
            )
            .color(colored::Color::Red)
            .bold()
            .into());
        }

        if model.velocity / model.frame_rate as f64
            > model.cell_size * SIMULATION_STEP_SIZE_FRACTION_OF_CELL
        {
            // We need to generate a frame every n:th step to get as close as possible to the frame rate
            // Will us the user or automatically determined step size as the base
            model.steps_per_frame =
                (model.velocity / model.frame_rate as f64 / model.step_size).ceil() as u64;

            // Give a warning that the step size is too large and we might not get the desired frame rate
            if model.steps_per_frame > 1 {
                eprintln!("{}",
                    format!("Warning: Simulation will generate frames every {} steps which gives an effective frame rate of {:.02} fps.",
                        model.steps_per_frame,
                        model.velocity / model.steps_per_frame as f64 / model.step_size ).color(colored::Color::Yellow).bold()
                );
            }
        } else {
            model.step_size = model.velocity / model.frame_rate as f64;
        }

        fs::create_dir_all(&model.frames_dir).map_err(|e| {
            format!(
                "{}: {}",
                format!(
                    "Failed to create output frame directory '{}'",
                    model.frames_dir
                )
                .color(colored::Color::Red)
                .bold(),
                e
            )
        })?;
    }

    // Use the user-defined start position
    model.start_x = args.start_x;
    model.start_y = args.start_y;

    // Setup the initial direction of movement based on user input or randomize it
    let (current_dir_x, current_dir_y, angle_deg) = set_initial_direction(args, rng);
    model.start_dir_x = current_dir_x;
    model.start_dir_y = current_dir_y;
    model.start_angle_deg = angle_deg;

    // Setup the paper size in mm
    if let Some((width_mm, height_mm)) = args.paper_size.get_size_mm() {
        model.image_width_mm = width_mm as u32;
        model.image_height_mm = height_mm as u32;
    } else {
        return Err("Unknown paper size.".to_string().into());
    }

    Ok(model)
}

pub fn setup_grid_size(model: &mut SimModel) -> Result<(), Box<dyn std::error::Error + 'static>> {
    model.grid_cells_x = (model.grid_width / model.cell_size).ceil() as usize;
    model.grid_cells_y = (model.grid_height / model.cell_size).ceil() as usize;
    if model.grid_cells_x * model.grid_cells_y > 10_000_000 {
        return Err("Grid size is too large (>10,000,000).".into());
    }
    model.grid_width = model.grid_cells_x as f64 * model.cell_size;
    model.grid_height = model.grid_cells_y as f64 * model.cell_size;
    model.bb = boundingbox::BoundingBox::init(model.grid_width, model.grid_height, model.radius);

    if model.cutter_type == cuttertype::CutterType::Blade
        && (model.cell_size > model.blade_len / 1.5)
    {
        return Err("Cell size must be < 3/2 of blade length".into());
    }

    if model.step_size >= model.cell_size {
        return Err(format!(
            "Step size {} must be smaller than square size {}",
            model.step_size, model.cell_size
        )
        .into());
    }

    // Calculate the number of cells in the grid based on the square size
    // Round the number of cells to the nearest whole number

    // Sanity check for grid size

    // Re-adjust the width/height to make sure it is a whole number of cells

    // // Initialize the grid with the calculated number of cells and cell size
    // model.grid = Some(Grid::new(
    //     model.grid_cells_x,
    //     model.grid_cells_y,
    //     model.cell_size,
    // ));
    Ok(())
}
