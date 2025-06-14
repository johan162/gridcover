use clap::Parser;
use crate::model::CutterType;

#[derive(Parser, Debug)]
#[command(author, version, about = "Grid coverage simulation")]
pub struct Args {

    /// Output image file name
    #[arg(short = 'o', name = "IMAGE-FILE-NAME", default_value = None,
           value_name = "IMAGE-FILE-NAME")]
    pub image_file_name: Option<String>,

    /// Simulation step size in units if not specified will be calculated from the square size
    #[arg(long, short = 'z', default_value_t = 0.0)]
    pub step_size: f64,

    /// Radius of the circle 
    #[arg(short = 'r', long, default_value_t = 0.2)]
    pub radius: f64,

    /// Length of knife blade
    #[arg(long, short = 'l', default_value_t = 0.05)]
    pub blade_len: f64,

    /// Grid width in cells 
    #[arg(short = 'w', long, default_value_t = 500)]
    pub grid_width: usize,

    /// Grid height in cells
    #[arg(short = 'g', long, default_value_t = 500)]
    pub grid_height: usize,

    /// Size of each grid square
    #[arg(short = 's', long, default_value_t = 0.0)]
    pub square_size: f64,

    /// Starting X coordinate for the circle center 
    #[arg(short = 'x', long, default_value_t = 0.0)]
    pub start_x: f64,

    /// Starting Y coordinate for the circle center 
    #[arg(short = 'y', long, default_value_t = 0.0)]
    pub start_y: f64,

    /// Movement velocity in units/second 
    #[arg(short = 'v', long, default_value_t = 0.5)]
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
    #[arg(long, default_value_t = false, action = clap::ArgAction::Set)]
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

    /// Maximum simulated time when to stop
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
    #[arg(long, short='P', default_value_t = true, action = clap::ArgAction::Set)]
    pub parallel: bool,

    /// Random seed for the simulation to be able to reproduce results
    /// If not specified, a random seed will be generated
    #[arg(long, short = 'S', default_value_t = 0)]
    pub random_seed: u64,

    /// Image output width in mm (50-1000)
    #[arg(long, default_value_t = 200,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<usize, String> {
            let val: usize = s.parse().map_err(|_| "Not a valid width value".to_string())?;
            if val >= 50 && val <= 1000 {
                Ok(val)
            } else {
                Err(format!("Image width must be between 50mm and 1000mm, got {}", val))
            }
        })
    )]
    pub image_width_mm: usize,

    /// Image output height in mm (50-1000)
    #[arg(long, default_value_t = 200,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<usize, String> {
            let val: usize = s.parse().map_err(|_| "Not a valid height value".to_string())?;
            if val >= 50 && val <= 1000 {
                Ok(val)
            } else {
                Err(format!("Image height must be between 50mm and 1000mm, got {}", val))
            }
        })
    )]
    pub image_height_mm: usize,

    /// Add option to turn centerpoint tracking on or off
    #[arg(long, short = 'C', default_value_t = true, action = clap::ArgAction::Set)]
    pub track_center: bool,

    /// Show progress bar during simulation (default: true)
    #[arg(long, short = 'R', default_value_t = true, action = clap::ArgAction::Set)]
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
            if val >= 1.0 && val <= 720.0 {
                Ok(val)
            } else {
                Err(format!("Battery value must be between 1 and 720 minutes (12h), got {}", val))
            }
        })
    )]
    pub battery_run_time: f64,

    /// Battery charging time in minutes for the cutter when it runs out
    #[arg(long, short = 'A', default_value_t = 0.0,
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
  
}
