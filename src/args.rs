use clap::Parser;
use crate::model::{cuttertype::CutterType, papersize::PaperSize};
use serde::{Serialize, Deserialize};

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;


impl Default for Args {
    fn default() -> Self {
        // This will use clap's default values
        Args::parse_from(std::iter::empty::<&str>())
    }
}


/// Writes the program arguments to a TOML formatted file
///
/// # Arguments
///
/// * `args` - The program arguments structure
/// * `file_path` - Path where the TOML file should be saved
///
/// # Returns
///
/// * `Ok(())` if the file was successfully written
/// * `Err(e)` if there was an error during serialization or file writing
pub fn write_args_to_file<T: Serialize>(
    args: &T,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Convert the args struct to a TOML string
    let toml_string = toml::to_string_pretty(&args)?;

    // Create the file and write the TOML string to it
    let path = Path::new(file_path);
    let mut file = File::create(path)?;
    file.write_all(toml_string.as_bytes())?;

    Ok(())
}

/// Reads program arguments from a TOML formatted file
///
/// # Arguments
///
/// * `file_path` - Path of the TOML file to read
///
/// # Returns
///
/// * `Ok(T)` containing the parsed arguments structure if successful
/// * `Err(e)` if there was an error during file reading or deserialization
pub fn read_args_from_file<T>(file_path: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'de> Deserialize<'de> + Default,
{
    // Open the file
    let path = Path::new(file_path);
    if !path.exists() {
        // Return default if file doesn't exist
        return Ok(T::default());
    }

    let mut file = File::open(path)?;
    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string)?;

    // Parse the TOML string to the args structure
    let args: T = toml::from_str(&toml_string)?;

    Ok(args)
}


#[derive(Parser, Debug, Serialize, Deserialize)]
#[command(author, version, about = "Grid coverage simulation")]
#[serde(default)] 
pub struct Args {

    /// Output image file name
    #[arg(short = 'o', default_value = None, value_name = "IMAGE-FILE-NAME")]
    pub image_file_name: Option<String>,

    /// Write program arguments file in TOML format
    #[arg(long, default_value = None, value_name = "ARGS-FILE-NAME")]
    pub args_write_file_name: Option<String>,

    /// Read program arguments from a TOML file
    #[arg(long, short = 'i', default_value = None, value_name = "ARGS-FILE-NAME")]
    pub args_read_file_name: Option<String>,

    /// Simulation step size in units if not specified will be calculated from the square size
    #[arg(long, short = 'z', default_value_t = 0.0)]
    pub step_size: f64,

    /// Radius of the circle 
    #[arg(short = 'r', long, default_value_t = 0.15,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid radius value".to_string())?;
            if val > 0.0 && val <= 10.0 {
                Ok(val)
            } else {
                Err(format!("Radius must be between 0.01 and 10.0, got {}", val))
            }
        })
    )]
    pub radius: f64,

    /// Length of knife blade
    #[arg(long, short = 'l', default_value_t = 0.05)]
    pub blade_len: f64,

    /// Width in units of the grid
    #[arg(short = 'W', long, default_value_t = 0.0)]
    pub grid_width: f64,

    /// Height in units of the grid
    #[arg(short = 'H', long, default_value_t = 0.0)]
    pub grid_height: f64,

    /// Size of each grid square
    #[arg(short = 's', long, default_value_t = -1.0)]
    pub cell_size: f64,

    /// Starting X coordinate for the circle center 
    #[arg(short = 'x', long, default_value_t = -1.0)]
    pub start_x: f64,

    /// Starting Y coordinate for the circle center 
    #[arg(short = 'y', long, default_value_t = 0.0)]
    pub start_y: f64,

    /// Movement velocity in units/second 
    #[arg(short = 'v', long, default_value_t = 0.3)]
    pub velocity: f64,

    /// Direction X component 
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    pub start_dir_x: f64,

    /// Direction Y component 
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    pub start_dir_y: f64,

    /// Use perturbation angle for direction changes at bounce 
    #[arg(long, short='p', default_value_t = true, action = clap::ArgAction::Set)]
    pub perturb: bool,

    /// Use perturbation randomly while moving in a straight line
    #[arg(long, short='k', default_value_t = false, action = clap::ArgAction::Set)]
    pub perturb_segment: bool,

    /// Perturb segment percent chance per cell travelled
    #[arg(long, default_value_t = 0.5,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid perturbation percent".to_string())?;
            if val >= 0.0 && val <= 30.0 {
                Ok(val)
            } else {
                Err(format!("Value must be between 0% and 30%, got {}", val))
            }
        })
    )]
    pub perturb_segment_percent: f64,

    /// Maximum number of bounces before ending simulation 
    #[arg(short = 'b', long, default_value_t = 0)]
    pub stop_bounces: usize,

    /// Maximum simulated time when to stop in seconds
    #[arg(long, short = 't', default_value_t = 0.0)]
    pub stop_time: f64,

    /// Stop when we have reached this coverage percentage
    /// This is a soft limit, the simulation will still run until the specified bounces or time is reached if specified
    #[arg(long, short = 'c', default_value_t = 0.0,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid coverage percent".to_string())?;
            if val >= 0.0 && val <= 99.9 {
                Ok(val)
            } else {
                Err(format!("Value must be between 1% and 99%, got {}", val))
            }
        })
    )]
    pub stop_coverage: f64,

    /// Stop when we have reached the specified number of simulation steps 
    #[arg(long, short = 'm', default_value_t = 0)]
    pub stop_simsteps: u64,

    /// Stop when we have reached the specified distance covered 
    #[arg(long, short = 'd', default_value_t = 0.0)]
    pub stop_distance: f64,

    /// Verbosity during simulation 
    #[arg(long, default_value_t = 0)]
    pub verbosity: usize,

    /// Use parallel processing to speed up simulation 
    #[arg(long, short='P', default_value_t = false, action = clap::ArgAction::Set)]
    pub parallel: bool,

    /// Random seed for the simulation to be able to reproduce results
    /// If not specified, a random seed will be generated
    #[arg(long, short = 'S', default_value_t = 0)]
    pub random_seed: u64,

    /// Image output width in mm (50-2000)
    #[arg(long, default_value_t = 210,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<u32, String> {
            let val: u32 = s.parse().map_err(|_| "Width value illegal".to_string())?;
            if val ==0 || val >= 50 && val <= 2000 {
                Ok(val)
            } else {
                Err(format!("Image width must be between 50mm and 2000mm, got {}", val))
            }
        })
    )]
    pub image_width_mm: u32,

    /// Image output height in mm (50-2000)
    #[arg(long, default_value_t = 297,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<u32, String> {
            let val: u32 = s.parse().map_err(|_| "Height value illegal".to_string())?;
            if val ==0 || val >= 50 && val <= 2000 {
                Ok(val)
            } else {
                Err(format!("Image height must be between 50mm and 2000mm, got {}", val))
            }
        })
    )]
    pub image_height_mm: u32,

    /// Image paper size to use for the output image
    #[arg(long, short = 'Z', value_name = "PAPER-SIZE",
         value_parser = clap::builder::EnumValueParser::<PaperSize>::new(),
         help = "Paper size to use for the output image. Options: 'A0', 'A1', 'A2', 'A3', 'A4', 'A5', 'Letter', 'Legal'.",
         value_hint = clap::ValueHint::Other,
         ignore_case = true,
         value_enum, default_value_t = PaperSize::A4)]
    pub paper_size: PaperSize,

    /// Add option to turn centerpoint tracking on or off
    #[arg(long, short = 'C', default_value_t = false, action = clap::ArgAction::Set)]
    pub track_center: bool,

    /// Show progress bar during simulation (default: true)
    #[arg(long, short = 'R', default_value_t = false, action = clap::ArgAction::Set)]
    pub show_progress: bool,

    /// Cutter type to use for the simulation (default: "blade")
    #[arg(long, short = 'T',  value_name = "CUTTER-TYPE",
         value_parser = clap::builder::EnumValueParser::<CutterType>::new(),
         help = "Cutter type to use for the simulation. Options: 'blade', 'circular'.",
         value_hint = clap::ValueHint::Other,
         ignore_case = true,  
        value_enum, default_value_t = CutterType::Blade)]
    pub cutter_type: CutterType,

    /// DPI setting for image output (default: 300)
    #[arg(long, short = 'D', default_value_t = 300,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<u32, String> {
            let val: u32 = s.parse().map_err(|_| "Not a valid DPI value".to_string())?;
            if val >= 72 && val <= 1200 {
                Ok(val)
            } else {
                Err(format!("DPI must be between 72 and 1200, got {}", val))
            }
        })
    )]
    pub dpi: u32,

    /// Print results as a json object
    #[arg(long, short = 'J', default_value_t = false, action = clap::ArgAction::Set)]
    pub json_output: bool,

    /// Battery duration in minutes for the cutter
    #[arg(long, short = 'B', default_value_t = 0.0,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid battery value".to_string())?;
            if val >= 0.0 && val <= 720.0 {
                Ok(val)
            } else {
                Err(format!("Battery value must be between 0 and 720 minutes (12h), got {}", val))
            }
        })
    )]
    pub battery_run_time: f64,

    /// Battery charging time in minutes for the cutter when it runs out
    #[arg(long, short = 'A', default_value_t = 120.0,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid charging time value".to_string())?;
            if val >= 1.0 && val <= 720.0 {
                Ok(val)
            } else {
                Err(format!("Charging time must be between 1 and 720 minutes (12h), got {}", val))
            }
        })
    )]
    pub battery_charge_time: f64,

    /// Path to map file with obstacles
    #[arg(short = 'M', long, default_value = None, value_name = "MAP-FILE")]
    pub map_file_name: Option<String>,

    /// Show or hide gridlines in the output image
    #[arg(long, short = 'G', default_value_t = false, action = clap::ArgAction::Set)]
    pub show_gridlines: bool,

    /// Store simulation results and model parameters in SQLite database file
    #[arg(long, short = 'Q', default_value = None, value_name = "DATABASE-FILE")]
    pub database_file: Option<String>,

    /// Quiet, no output at all
    #[arg(long, short = 'q', default_value_t = false, action = clap::ArgAction::Set)]
    pub quiet: bool,

    /// Generate frames for an animation
    #[arg(long, short = 'f', default_value_t = false, action = clap::ArgAction::Set)]
    pub generate_frames: bool,

    /// Specify frame-rate for the animation
    #[arg(long, short = 'F', default_value_t = 5,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<u64, String> {
            let val: u64 = s.parse().map_err(|_| "Not a valid frame rate value".to_string())?;
            if val >= 5 && val <= 30 {
                Ok(val)
            } else {
                Err(format!("Frame rate must be between 5 and 30, got {}", val))
            }
        })
    )]  
    pub frame_rate: u64,

    // Specify directory to save frames for animation
    #[arg(long, default_value = "frames_dir", value_name = "FRAMES-DIR")]
    pub frames_dir: String,

    /// Generate an animation video from the frames
    #[arg(long, short = 'a', default_value_t = false, action = clap::ArgAction::Set)]
    pub create_animation: bool,

    /// Animation file name
    #[arg(long, default_value = "cutter_sim.mp4", value_name = "ANIMATION-FILE-NAME")]
    pub animation_file_name: String,

    /// Animation speedup factor
    #[arg(long, short = 'U', default_value_t = 25, value_name = "ANIMATION-SPEEDUP-FACTOR")]
    pub animation_speedup: u64,

    /// Use HW assisted encoding for the animation. This is only available on macOS and Linux
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub hw_encoding: bool,

    /// Delete frames after animation has been created
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub delete_frames: bool,

    /// Color theme to use as an string, possible values: "default", "green30", "blue", "high_contrast", "pure_green", "gray_green"
    #[arg(long, default_value = None, value_name = "COLOR-THEME")]
    pub color_theme: Option<String>,

    /// Add simulation of wheel slippage which will cause the cutter to not follow the path exactly
    #[arg(long, default_value_t = false, action = clap::ArgAction::Set)]
    pub wheel_slippage: bool,

    /// Slippage probability per slippage_activation_check_distance
    #[arg(long, default_value_t = 0.1,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid slippage probability value".to_string())?;
            if val >= 0.0 && val <= 1.0 {
                Ok(val)
            } else {
                Err(format!("Slippage probability must be between 0.0 and 1.0, got {}", val))
            }
        })
    )]
    pub slippage_probability: f64,

    /// Slippage activation min distance in units
    #[arg(long, default_value_t = 20.0, value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
        let val: f64 = s.parse().map_err(|_| "Not a valid slippage min distance value".to_string())?;
        if val >= 1.0 {
            Ok(val)
        } else {
            Err(format!("Slippage min distance must be non-negative, got {}", val))
        }
    }))]
    pub slippage_min_distance: f64,

    /// Slippage activation max distance in units
    #[arg(long, default_value_t = 200.0, value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
        let val: f64 = s.parse().map_err(|_| "Not a valid slippage max distance value".to_string())?;
        if val >= 1.0 {
            Ok(val)
        } else {
            Err(format!("Slippage max distance must be non-negative, got {}", val))
        }
    }))]
    pub slippage_max_distance: f64,

    /// Slippage min angle in degrees to adjust as slippage per defined steps
    #[arg(long, default_value_t = 5.0,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid slippage radius min value".to_string())?;
            if val >= 1.0 && val <= 50.0 {
                Ok(val)
            } else {
                Err(format!("Slippage radius min must be between 1.0 and 50.0, got {}", val))
            }
        })
    )]
    pub slippage_radius_min: f64,

    /// Slippage max angle in degrees to adjust as slippage per defined steps
    #[arg(long, default_value_t = 20.0,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid slippage radius max value".to_string())?;
            if val >= 1.0 && val <= 50.0 {
                Ok(val)
            } else {
                Err(format!("Slippage radius max must be between 5.0 and 50.0, got {}", val))
            }
        })
    )]
    pub slippage_radius_max: f64,

    /// Check if we should activate slippage every this units travelled
    #[arg(long, default_value_t = 20.0)]
    pub slippage_check_activation_distance: f64,

    /// While in slippage mode adjust the angle every n:th units travelled
    #[arg(long, default_value_t = 0.2,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid slippage adjustment step value".to_string())?;
            if val >= 0.1 && val <= 10.0 {
                Ok(val)
            } else {
                Err(format!("Slippage adjustment step must be between 0.1 and 10.0, got {}", val))
            }
        })
    )]
    pub slippage_adjustment_step: f64,

    /// Wheel inbalance simulation, this will cause the cutter to not follow the straight path exactly
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub wheel_inbalance: bool,

    /// We model wheel inbalance as the cutter turning in a random radius between a min/max value
    #[arg(long, default_value_t = 20.0,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid wheel inbalance radius min value".to_string())?;
            if val >= 1.0 && val <= 1000.0 {
                Ok(val)
            } else {
                Err(format!("Wheel inbalance radius min must be between 1.0 and 1000.0, got {}", val))
            }
        })
    )]
    pub wheel_inbalance_radius_min: f64,

    /// We model wheel inbalance as the cutter turning in a random radius between a min/max value
    #[arg(long, default_value_t = 100.0,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid wheel inbalance radius max value".to_string())?;
            if val >= 1.0 && val <= 1000.0 {
                Ok(val)
            } else {
                Err(format!("Wheel inbalance radius max must be between 1.0 and 1000.0, got {}", val))
            }
        })
    )]
    pub wheel_inbalance_radius_max: f64,     

    /// Wheel inbalance adjustment distance in units
    #[arg(long, default_value_t = 0.2,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid wheel inbalance adjustment distance value".to_string())?;
            if val >= 0.1 && val <= 10.0 {
                Ok(val)
            } else {
                Err(format!("Wheel inbalance adjustment distance must be between 0.1 and 10.0, got {}", val))
            }
        })
    )]
    pub wheel_inbalance_adjustment_step: f64,

    /// Show the quad-tree structure in the output image
    #[arg(long, default_value_t = false, action = clap::ArgAction::Set)]
    pub show_quad_tree: bool,

    /// Min quad tree node size in multiples of cutter radius
    #[arg(long, default_value_t = 8.0,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<f64, String> {
            let val: f64 = s.parse().map_err(|_| "Not a valid min qnode size value".to_string())?;
            if val >= 3.0 && val <= 30.0 {
                Ok(val)
            } else {
                Err(format!("Min quad node size must be between 3.0 and 30.0 times the radius, got {}", val))
            }
        })
    )]
    pub min_qnode_size: f64,

    /// Enable the use of a quad-tree for faster collision detection
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub use_quad_tree: bool,

    /// Save the quad-tree structure to a file with name based on the map file name
    #[arg(long, default_value_t = false, action = clap::ArgAction::Set)]
    pub save_quad_tree: bool,

    /// Decide if label with sim-time and coverage should be added to the output image
    #[arg(long, default_value_t = true, action = clap::ArgAction::Set)]
    pub show_image_label: bool,

    /// Generate model.json and result.json as two files at end of simulation and
    /// only print the short version to the console.
    #[arg(short = 'X', long, default_value_t = false, action = clap::ArgAction::Set)]
    pub generate_json_files: bool,
}


impl Args {
    /// Merge another Args into self, preferring self's non-default values over other's.
    pub fn merge_with(self, other: Args) -> Args {
        Args {
            image_file_name: self.image_file_name.or(other.image_file_name),
            args_write_file_name: self.args_write_file_name.or(other.args_write_file_name),
            args_read_file_name: self.args_read_file_name.or(other.args_read_file_name),
            step_size: if self.step_size != 0.0 { self.step_size } else { other.step_size },
            radius: if self.radius != 0.15 { self.radius } else { other.radius },
            blade_len: if self.blade_len != 0.05 { self.blade_len } else { other.blade_len },
            grid_width: if self.grid_width > 0.0 { self.grid_width } else { other.grid_width },
            grid_height: if self.grid_height > 0.0 { self.grid_height } else { other.grid_height },
            cell_size: if self.cell_size > 0.0 { self.cell_size } else { other.cell_size },
            start_x: if self.start_x > 0.0 { self.start_x } else { other.start_x },
            start_y: if self.start_y > 0.0 { self.start_y } else { other.start_y },
            velocity: if self.velocity != 0.3 { self.velocity } else { other.velocity },
            start_dir_x: if self.start_dir_x != 0.0 { self.start_dir_x } else { other.start_dir_x },
            start_dir_y: if self.start_dir_y != 0.0 { self.start_dir_y } else { other.start_dir_y },
            perturb: if !self.perturb { self.perturb } else { other.perturb },
            perturb_segment: if self.perturb_segment { self.perturb_segment } else { other.perturb_segment },
            perturb_segment_percent: if self.perturb_segment_percent != 0.5 { self.perturb_segment_percent } else { other.perturb_segment_percent },
            stop_bounces: if self.stop_bounces > 0 { self.stop_bounces } else { other.stop_bounces },
            stop_time: if self.stop_time > 0.0 { self.stop_time } else { other.stop_time },
            stop_coverage: if self.stop_coverage > 0.0 { self.stop_coverage } else { other.stop_coverage },
            stop_simsteps: if self.stop_simsteps > 0 { self.stop_simsteps } else { other.stop_simsteps },
            stop_distance: if self.stop_distance > 0.0 { self.stop_distance } else { other.stop_distance },
            verbosity: if self.verbosity > 0 { self.verbosity } else { other.verbosity },
            parallel: if !self.parallel { self.parallel } else { other.parallel },
            random_seed: if self.random_seed > 0 { self.random_seed } else { other.random_seed },
            image_width_mm: if self.image_width_mm != 210 { self.image_width_mm } else { other.image_width_mm },
            image_height_mm: if self.image_height_mm != 297 { self.image_height_mm } else { other.image_height_mm },
            track_center: if !self.track_center { self.track_center } else { other.track_center },
            show_progress: if !self.show_progress { self.show_progress } else { other.show_progress },
            cutter_type: if self.cutter_type != CutterType::Blade { self.cutter_type } else { other.cutter_type },
            dpi: if self.dpi != 300 { self.dpi } else { other.dpi },
            json_output: if self.json_output { self.json_output } else { other.json_output },
            battery_run_time: if self.battery_run_time > 0.0 { self.battery_run_time } else { other.battery_run_time },
            battery_charge_time: if self.battery_charge_time != 120.0 { self.battery_charge_time } else { other.battery_charge_time },
            paper_size: if self.paper_size != PaperSize::A4 { self.paper_size } else { other.paper_size },
            map_file_name: self.map_file_name.or(other.map_file_name),
            show_gridlines: if self.show_gridlines { self.show_gridlines } else { other.show_gridlines },
            database_file: self.database_file.or(other.database_file),
            quiet: if self.quiet { self.quiet } else { other.quiet },
            generate_frames: if self.generate_frames { self.generate_frames } else { other.generate_frames },
            frame_rate: if self.frame_rate != 5 { self.frame_rate } else { other.frame_rate },
            frames_dir: if self.frames_dir != "frames_dir" { self.frames_dir } else { other.frames_dir },
            create_animation: if self.create_animation { self.create_animation } else { other.create_animation },
            animation_file_name: if self.animation_file_name != "cutter_sim.mp4" { self.animation_file_name } else { other.animation_file_name },
            hw_encoding: if self.hw_encoding { self.hw_encoding } else { other.hw_encoding },
            delete_frames: if self.delete_frames { self.delete_frames } else { other.delete_frames },
            animation_speedup: if self.animation_speedup != 1 { self.animation_speedup } else { other.animation_speedup },
            color_theme: self.color_theme.or(other.color_theme),
            wheel_slippage: if self.wheel_slippage { self.wheel_slippage } else { other.wheel_slippage },
            slippage_probability: if self.slippage_probability != 0.02 { self.slippage_probability } else { other.slippage_probability },
            slippage_min_distance: if self.slippage_min_distance != 20.0 { self.slippage_min_distance } else { other.slippage_min_distance },
            slippage_max_distance: if self.slippage_max_distance != 100.0 { self.slippage_max_distance } else { other.slippage_max_distance },
            slippage_radius_min: if self.slippage_radius_min != 5.0 { self.slippage_radius_min } else { other.slippage_radius_min },
            slippage_radius_max: if self.slippage_radius_max != 20.0 { self.slippage_radius_max } else { other.slippage_radius_max },
            slippage_check_activation_distance: if self.slippage_check_activation_distance != 10.0 { self.slippage_check_activation_distance } else { other.slippage_check_activation_distance },
            slippage_adjustment_step: if self.slippage_adjustment_step != 0.2 { self.slippage_adjustment_step } else { other.slippage_adjustment_step },
            wheel_inbalance: if self.wheel_inbalance { self.wheel_inbalance } else { other.wheel_inbalance },
            wheel_inbalance_radius_min: if self.wheel_inbalance_radius_min != 40.0 { self.wheel_inbalance_radius_min } else { other.wheel_inbalance_radius_min },
            wheel_inbalance_radius_max: if self.wheel_inbalance_radius_max != 150.0 { self.wheel_inbalance_radius_max } else { other.wheel_inbalance_radius_max },
            wheel_inbalance_adjustment_step: if self.wheel_inbalance_adjustment_step != 0.2 { self.wheel_inbalance_adjustment_step } else { other.wheel_inbalance_adjustment_step },
            show_quad_tree: if self.show_quad_tree { self.show_quad_tree } else { other.show_quad_tree },
            min_qnode_size: if self.min_qnode_size != 8.0 { self.min_qnode_size } else { other.min_qnode_size },
            use_quad_tree: if self.use_quad_tree { self.use_quad_tree } else { other.use_quad_tree },
            save_quad_tree: if self.save_quad_tree { self.save_quad_tree } else { other.save_quad_tree },
            show_image_label: if self.show_image_label { self.show_image_label } else { other.show_image_label },
            generate_json_files: if self.generate_json_files { self.generate_json_files } else { other.generate_json_files },
        }
    }
}
