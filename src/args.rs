use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "Grid coverage simulation")]
pub struct Args {

    /// Output image file name
    #[arg(name = "IMAGE-FILE-NAME", default_value = None,
           value_name = "IMAGE-FILE-NAME")]
    pub output_file: Option<String>,

    /// Radius of the circle (default: 0.3)
    #[arg(short = 'r', long, default_value_t = 0.3)]
    pub radius: f64,

    /// Grid width in cells (default: 200)
    #[arg(short = 'w', long, default_value_t = 500)]
    pub width: usize,

    /// Grid height in cells (default: 200)
    #[arg(short = 'g', long, default_value_t = 500)]
    pub height: usize,

    /// Size of each grid square (default: 0.2)
    #[arg(short = 's', long, default_value_t = 0.1)]
    pub square_size: f64,

    /// Starting X coordinate for the circle center (default: 0)
    #[arg(short = 'x', long, default_value_t = 0.0)]
    pub start_x: f64,

    /// Starting Y coordinate for the circle center (default: 0)
    #[arg(short = 'y', long, default_value_t = 0.0)]
    pub start_y: f64,

    /// Movement velocity in units/second (default: 0.5)
    #[arg(short = 'v', long, default_value_t = 0.5)]
    pub velocity: f64,

    /// Direction X component (default: random)
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    pub dir_x: f64,

    /// Direction Y component (default: random)
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    pub dir_y: f64,

    /// Use perturbation angle for direction changes (default: true)
    #[arg(long, short='p', default_value_t = true, action = clap::ArgAction::Set)]
    pub perturb: bool,

    /// Maximum number of bounces before ending simulation (default: 10)
    #[arg(short = 'b', long, default_value_t = 0)]
    pub stop_bounces: usize,

    /// Maximum simulated time when to stop (0 use bounce count)
    #[arg(long, short = 't', default_value_t = 0.0)]
    pub stop_time: f64,

    /// Stop when we have reached this coverage percentage (default: 0.0, means no limit)
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

    /// Stop when we have reached the specified number of simulation steps (default: 0, means no limit)
    #[arg(long, short = 'm', default_value_t = 0)]
    pub stop_simsteps: u64,

    /// Stop when we have reached the specified distance covered (default: 0.0, means no limit)
    #[arg(long, short = 'd', default_value_t = 0.0)]
    pub stop_distance: f64,

    /// Verbosity during simulation (default: 0)
    #[arg(long, default_value_t = 0)]
    pub verbosity: usize,

     /// Use parallel processing to speed up simulation (default: true)
    #[arg(long, short='P', default_value_t = true, action = clap::ArgAction::Set)]
    pub parallel: bool,

    /// Random seed for the simulation to be able to reproduce results
    /// If not specified, a random seed will be generated
    #[arg(long, short = 'S', default_value_t = 0)]
    pub random_seed: u64,

    /// Image output width in mm (50-500)
    #[arg(long, default_value_t = 200,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<usize, String> {
            let val: usize = s.parse().map_err(|_| "Not a valid width value".to_string())?;
            if val >= 50 && val <= 500 {
                Ok(val)
            } else {
                Err(format!("Image width must be between 50mm and 500mm, got {}", val))
            }
        })
    )]
    pub image_width: usize,

    /// Image output height in mm (50-500)
    #[arg(long, default_value_t = 200,
        value_parser = clap::builder::ValueParser::new(|s: &str| -> Result<usize, String> {
            let val: usize = s.parse().map_err(|_| "Not a valid height value".to_string())?;
            if val >= 50 && val <= 500 {
                Ok(val)
            } else {
                Err(format!("Image height must be between 50mm and 500mm, got {}", val))
            }
        })
    )]
    pub image_height: usize,

    /// Add option to turn centerpoint tracking on or off
    #[arg(long, short = 'C', default_value_t = true, action = clap::ArgAction::Set)]
    pub track_center: bool,

    /// Show progress bar during simulation (default: true)
    #[arg(long, short = 'R', default_value_t = true, action = clap::ArgAction::Set)]
    pub show_progress: bool,

}
