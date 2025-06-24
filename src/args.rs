use clap::Parser;
use crate::model::{cuttertype::CutterType, papersize::PaperSize};
use serde::{Serialize, Deserialize};

impl Default for Args {
    fn default() -> Self {
        // This will use clap's default values
        Args::parse_from(std::iter::empty::<&str>())
    }
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
    #[arg(short = 'r', long, default_value_t = 0.2)]
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
    pub square_size: f64,

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
    pub dir_x: f64,

    /// Direction Y component 
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    pub dir_y: f64,

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
}


impl Args {
    /// Merge another Args into self, preferring self's non-default values over other's.
    pub fn merge_with(self, other: Args) -> Args {
        Args {
            image_file_name: self.image_file_name.or(other.image_file_name),
            args_write_file_name: self.args_write_file_name.or(other.args_write_file_name),
            args_read_file_name: self.args_read_file_name.or(other.args_read_file_name),
            step_size: if self.step_size != 0.0 { self.step_size } else { other.step_size },
            radius: if self.radius != 0.2 { self.radius } else { other.radius },
            blade_len: if self.blade_len != 0.05 { self.blade_len } else { other.blade_len },
            grid_width: if self.grid_width > 0.0 { self.grid_width } else { other.grid_width },
            grid_height: if self.grid_height > 0.0 { self.grid_height } else { other.grid_height },
            square_size: if self.square_size > 0.0 { self.square_size } else { other.square_size },
            start_x: if self.start_x > 0.0 { self.start_x } else { other.start_x },
            start_y: if self.start_y > 0.0 { self.start_y } else { other.start_y },
            velocity: if self.velocity != 0.3 { self.velocity } else { other.velocity },
            dir_x: if self.dir_x != 0.0 { self.dir_x } else { other.dir_x },
            dir_y: if self.dir_y != 0.0 { self.dir_y } else { other.dir_y },
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
        }
    }
}
