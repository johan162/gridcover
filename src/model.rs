use crate::args;
use chrono::Duration;
use clap::ValueEnum;
use colored::Colorize;
use rand::Rng;
use thousands::Separable;
use serde_json::json;

const DEFAULT_IMAGE_FILE_NAME: &str = "coverage_grid.png";
const MIN_RADIUS: f64 = 0.15; // Minimum radius for the circle

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
  pub min_x: f64,
  pub max_x: f64,
  pub min_y: f64,
  pub max_y: f64,
}

impl BoundingBox {
    fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
    pub fn init(width: usize, height: usize, radius: f64, square_size: f64) -> Self {
        let min_x = radius;
        let max_x = (width as f64) * square_size - radius;
        let min_y = radius;
        let max_y = (height as f64) * square_size - radius;
        Self::new(min_x, max_x, min_y, max_y)
    }
    pub fn limit_x(&self, x: f64) -> f64 {
        x.max(self.min_x).min(self.max_x)
    }
    pub fn limit_y(&self, y: f64) -> f64 {
        y.max(self.min_y).min(self.max_y)
    }
}


// A struct to track information about cell coverage
#[derive(Debug, Clone, Copy)]
pub struct CoverageInfo {
    pub covered: bool,
    pub bounce_number: usize, // Which bounce iteration covered this cell
    pub times_visited: usize, // How many times this cell was covered
}

impl CoverageInfo {
    pub fn new() -> Self {
        Self {
            covered: false,
            bounce_number: 0,
            times_visited: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum CutterType {
    Blade,
    Circular,
}

/// Iplement clap::validate::ValueEnum for CutterType
impl CutterType {
    pub fn as_str(&self) -> &str {
        match self {
            CutterType::Circular => "circular",
            CutterType::Blade => "blade",
        }
    }
}

impl std::str::FromStr for CutterType {
    type Err = String;
    #[allow(dead_code)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "circular" => Ok(CutterType::Circular),
            "blade" => Ok(CutterType::Blade),
            _ => Err(format!("Invalid cutter type: {s}")),
        }
    }
}

#[derive(Debug)]
pub struct SimModel {
    pub start_x: f64,
    pub start_y: f64,
    pub start_dir_x: f64,
    pub start_dir_y: f64,
    pub start_angle_deg: f64,
    pub step_size: f64,
    pub radius: f64,
    pub grid_width: usize,
    pub grid_height: usize,
    pub square_size: f64,
    pub velocity: f64,
    pub sim_time_elapsed: f64,
    pub bounce_count: usize,
    pub coverage_percent: f64,
    pub coverage_count: usize,
    pub perturb: bool,
    pub sim_real_time: Duration,
    pub sim_steps: u64,
    pub distance_covered: f64,
    pub stop_coverage: f64,
    pub stop_time: f64,
    pub stop_bounces: usize,
    pub stop_simsteps: u64,
    pub stop_distance: f64,
    pub parallel: bool,
    pub image_width_mm: usize,
    pub image_height_mm: usize,
    pub image_file_name: String,
    pub verbosity: usize,
    pub track_center: bool,
    pub show_progress: bool,
    pub blade_len: f64,
    pub cutter_type: CutterType,
    pub dpi: u32,
    pub perturb_segment: bool,
    pub perturb_segment_percent: f64,
    pub coverage_grid: Vec<Vec<CoverageInfo>>,
    pub bb: BoundingBox,
    pub battery_run_time: f64, // Total time the battery can run in min
    pub battery_charge_time: f64, // Time it takes to fully charge the battery in min
    pub battery_charge_count: usize, // How many times the battery was charged
}

// Define a constant for the simulation step size factor
// This factor determines how many simulation steps are taken per square (cell) size
// To get a full coverage of squares we should always have a step size that is a fraction of the square size.
const SIMULATION_STEP_SIZE_FRACTION_OF_SQUARE: f64 = 4.0 / 5.0;
const WIDTH_LABEL: usize = 25;

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
        grid_width: usize,
        grid_height: usize,
        square_size: f64,
        velocity: f64,
        stop_coverage: f64,
        stop_time: f64,
        stop_bounces: usize,
        stop_simsteps: u64,
        stop_distance: f64,
        parallel: bool,
        image_width_mm: usize,
        image_height_mm: usize,
        image_file_name: String,
        verbosity: usize,
        track_center: bool,
        show_progress: bool,
        blade_len: f64,
        cutter_type: CutterType,
        dpi: u32,
        perturb_segment: bool,
        perturb_segment_percent: f64,
        battery_run_time: f64,
        battery_charge_time: f64,
    ) -> Self {
        Self {
            start_x,
            start_y,
            start_dir_x,
            start_dir_y,
            start_angle_deg,
            step_size,
            radius,
            grid_width,
            grid_height,
            square_size,
            velocity,
            sim_time_elapsed: 0.0,
            bounce_count: 0,
            coverage_percent: 0.0,
            coverage_count: 0,
            perturb: true,
            sim_real_time: Duration::zero(),
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
            verbosity,
            track_center,
            show_progress,
            blade_len,
            cutter_type,
            dpi,
            perturb_segment,
            perturb_segment_percent,
            coverage_grid: vec![vec![CoverageInfo::new(); grid_width]; grid_height],
            bb: BoundingBox::init(grid_width, grid_height, radius, square_size),
            battery_run_time,
            battery_charge_time,
            battery_charge_count: 0,
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
            args.image_file_name
                .clone()
                .unwrap_or_else(|| DEFAULT_IMAGE_FILE_NAME.to_string()),
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
        )
    }

    pub fn print_simulation_parameters(&self) {
        println!(
            "{}",
            "Simulation Parameters:".color(colored::Color::Blue).bold()
        );
        println!("  {:<WIDTH_LABEL$}: {}", "Radius", self.radius);
        println!(
            "  {:<WIDTH_LABEL$}: {:.1} units",
            "Blade Length", self.blade_len
        );
        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Cutter Type",
            self.cutter_type.as_str()
        );
        // Battery information
        println!(
            "  {:<WIDTH_LABEL$}: {:.1} minutes",
            "Battery Run Time", self.battery_run_time
        );
        println!(
            "  {:<WIDTH_LABEL$}: {:.1} minutes",
            "Battery Charge Time", self.battery_charge_time
        );
        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Track Center",
            if self.track_center {
                "ENABLED"
            } else {
                "DISABLED"
            }
        );
        println!(
            "  {:<WIDTH_LABEL$}: ({:.2}, {:.2})",
            "Start Position", self.start_x, self.start_y
        );
        println!(
            "  {:<WIDTH_LABEL$}: ({:.2}, {:.2})",
            "Start Direction", self.start_dir_x, self.start_dir_y
        );
        println!(
            "  {:<WIDTH_LABEL$}: {:.2} degrees",
            "Start Angle", self.start_angle_deg
        );
        println!("  {:<WIDTH_LABEL$}: {:.2}", "Step Size", self.step_size);
        println!("  {:<WIDTH_LABEL$}: {:.2}", "Radius", self.radius);
        println!(
            "  {:<WIDTH_LABEL$}: {}x{} squares (={}x{} units)",
            "Grid Size",
            self.grid_width,
            self.grid_height,
            self.grid_width as f64 * self.square_size,
            self.grid_height as f64 * self.square_size
        );
        println!("  {:<WIDTH_LABEL$}: {:.4}", "Square Size", self.square_size);
        println!(
            "  {:<WIDTH_LABEL$}: {:.2} units/s",
            "Velocity", self.velocity
        );
        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Perturb at Bounces",
            if self.perturb { "ENABLED" } else { "DISABLED" }
        );
        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Perturb Segment",
            if self.perturb_segment { "ENABLED" } else { "DISABLED" }
        );
        println!(
            "  {:<WIDTH_LABEL$}: {:.1}%",
            "Perturb Segment Percent", self.perturb_segment_percent * 100.0
        );
        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Parallel processing",
            if self.parallel { "ENABLED" } else { "DISABLED" }
        );

        println!(
            "  {:<WIDTH_LABEL$}: {}x{} mm",
            "Image Size", self.image_width_mm, self.image_height_mm
        );

        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Image File Name", self.image_file_name
        );

        println!(
            "  {:<WIDTH_LABEL$}: {} dpi",
            "Image DPI", self.dpi
        );

        println!("  {:<WIDTH_LABEL$}: ", "Stop Conditions");
        if self.stop_bounces > 0 {
            println!("  {:<WIDTH_LABEL$}: - Bounces: {}", "", self.stop_bounces);
        }
        if self.stop_time > 0.0 {
            println!("  {:<WIDTH_LABEL$}: - Time: {:.1}s", "", self.stop_time);
        }
        if self.stop_coverage > 0.0 {
            println!(
                "  {:<WIDTH_LABEL$}: - Coverage: {:.1}%",
                "", self.stop_coverage
            );
        }
        if self.stop_simsteps > 0 {
            println!(
                "  {:<WIDTH_LABEL$}: - Simulation Steps: {}",
                "",
                self.stop_simsteps.separate_with_commas()
            );
        }
        if self.stop_distance > 0.0 {
            println!(
                "  {:<WIDTH_LABEL$}: - Distance: {:.1} units",
                "", self.stop_distance
            );
        }
    }

    pub fn print_simulation_results(&self) {
        println!(
            "\n{}",
            "Simulation Results:".color(colored::Color::Blue).bold()
        );
        println!(
            "  {:<WIDTH_LABEL$}: {:02}:{:02}",
            "Simulation time",
            self.sim_real_time.num_minutes(),
            self.sim_real_time.num_seconds() % 60
        );

        let total_seconds = self.sim_time_elapsed as u64;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        println!(
            "  {:<WIDTH_LABEL$}: {hours}:{minutes:02}:{seconds:02}",
            "Simulated elapsed time"
        );
        println!(
            "  {:<WIDTH_LABEL$}: {:.1}% ({} out of {} cells)",
            "Coverage",
            self.coverage_percent,
            self.coverage_count.separate_with_commas(),
            (self.grid_width * self.grid_height).separate_with_commas()
        );
        println!(
            "  {:<WIDTH_LABEL$}: {:.1} units",
            "Distance traveled:", self.distance_covered
        );
        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Number of bounces:", self.bounce_count
        );
        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Battery Charge Count", self.battery_charge_count
        );
        println!(
            "  {:<WIDTH_LABEL$}: {} (Step size = {:.2} units, Sim steps/cell = {})",
            "Total Simulation Steps",
            self.sim_steps.separate_with_commas(),
            self.step_size,
            (self.square_size / self.step_size).ceil()
        );
    }

    /// A version of print_simulation_results() that outputs results in JSON format
    pub fn print_simulation_results_json(&self) {
        let total_seconds = self.sim_time_elapsed as u64;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        let json = json!({
            "Simulation Result": {
                "Coverage": {
                    "Percent": self.coverage_percent,
                    "Count": self.coverage_count,
                    "Bounce count": self.bounce_count, 
                },
                "Cutter": {
                    "Type": self.cutter_type.as_str(),
                    "Blade Length": self.blade_len,
                    "Radius": self.radius,
                    "Velocity": self.velocity,
                    "Distance": self.distance_covered,
                    "Cells covered": (2.0*self.radius/self.square_size).floor() * (2.0*self.radius/self.square_size).floor(),
                    "Battery": {
                        "Run time": self.battery_run_time,
                        "Charge time": self.battery_charge_time,
                        "Charge count": self.battery_charge_count,
                    }
                },
                "Time": {
                     "Real": format!("{:02}:{:02}",
                        self.sim_real_time.num_minutes(),
                        self.sim_real_time.num_seconds() % 60),
                    "Simulation": format!("{:02}:{:02}:{:02}",
                        hours,
                        minutes,
                        seconds),
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
                    "Cells x": self.grid_width,
                    "Cells y": self.grid_height,
                    "Area": self.grid_width * self.grid_height,
                    "Cell size": self.square_size,
                    "Width": self.square_size * self.grid_width as f64,
                    "Height": self.square_size * self.grid_height as f64,
                },
                "Steps": {
                    "Total": self.sim_steps,
                    "Size": self.step_size,
                    "Per cell": self.square_size / self.step_size,
                }
            }
        });
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }

    #[allow(dead_code)]
    pub fn print_simulation_summary(&self) {
        self.print_simulation_parameters();
        self.print_simulation_results();
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
    let mut sim_model = SimModel::init(args);

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
    if sim_model.radius <= MIN_RADIUS {
        return Err(format!("Radius must be greater than {MIN_RADIUS} units").into());
    }

    if sim_model.square_size <= 0.0 {
        if sim_model.cutter_type == CutterType::Blade {
            sim_model.square_size = sim_model.blade_len / 3.0; // Default square size for blade cutter
        } else {
            sim_model.square_size = sim_model.radius / 3.0; // Default square size for circular cutter
        }
    }

    if sim_model.cutter_type == CutterType::Blade
        && (sim_model.square_size >= sim_model.blade_len / 3.0)
    {
        return Err("Square size must be < 1/3 of blade length".into());
    }

    if sim_model.step_size >= sim_model.square_size {
        return Err(format!(
            "Step size {} must be smaller than square size {}",
            sim_model.step_size, sim_model.square_size
        )
        .into());
    }

    // The size of the simulated grid must be at leat twice the radius
    if sim_model.grid_width as f64 * sim_model.square_size <= 2.0 * sim_model.radius
        || sim_model.grid_height as f64 * sim_model.square_size <= 2.0 * sim_model.radius
    {
        return Err(format!(
            "Grid size must be at least twice the diameter of the circle (Minimum {}x{} squares)",
            2 * (sim_model.radius / sim_model.square_size).ceil() as usize,
            2 * (sim_model.radius / sim_model.square_size).ceil() as usize
        )
        .into());
    }

    // If step size is not set then set it to 80% of the square size
    if sim_model.step_size <= 0.0 {
        sim_model.step_size = sim_model.square_size * SIMULATION_STEP_SIZE_FRACTION_OF_SQUARE;
    }

    // If both startx and start_y are zero, randomize the starting position
    if args.start_x == 0.0 && args.start_y == 0.0 {
        // Randomize start position within the bounding box
        // Print range
        // println!(
        //     "Randomizing start position within bounding box: ({:.2}, {:.2}) to ({:.2}, {:.2})",
        //     sim_model.radius,
        //     sim_model.radius,
        //     sim_model.grid_width as f64 * sim_model.square_size - sim_model.radius,
        //     sim_model.grid_height as f64 * sim_model.square_size - sim_model.radius
        // );
        sim_model.start_x = rng.random_range(
            sim_model.radius..(sim_model.grid_width as f64 * sim_model.square_size - sim_model.radius),
        );
        sim_model.start_y = rng.random_range(
            sim_model.radius..(sim_model.grid_height as f64 * sim_model.square_size - sim_model.radius),
        );
    } else {
        // Use the user-defined start position
        sim_model.start_x = args.start_x;
        sim_model.start_y = args.start_y;
    }

    // Setup the initial direction of movement based on user input or randomize it
    let (current_dir_x, current_dir_y, angle_deg) = set_initial_direction(args, rng);
    sim_model.start_dir_x = current_dir_x;
    sim_model.start_dir_y = current_dir_y;
    sim_model.start_angle_deg = angle_deg;

    Ok(sim_model)
}
