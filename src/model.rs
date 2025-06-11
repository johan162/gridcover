use crate::args;
use chrono::Duration;
use colored::Colorize;
use rand::Rng;
use thousands::Separable;

const DEFAULT_IMAGE_FILE_NAME: &str = "coverage_grid.png";

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

#[derive(Debug)]
pub struct SimModel {
    pub start_x: f64,
    pub start_y: f64,
    pub start_dir_x: f64,
    pub start_dir_y: f64,
    pub start_angle_deg: f64,
    pub step_size: f64,
    pub radius: f64,
    pub width: usize,
    pub height: usize,
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
    pub output_file: String, 
    pub show_progress: bool, 
}

// Define a constant for the simulation step size factor
// This factor determines how many simulation steps are taken per square (cell) size
// To get a full coverage of squares we should always have a step size that is a fraction of the square size. 
// Haaving a ste size 99% of the would be good enough but in order to not have to worry about rounding we simpply use a factor 10/9
const SIMULATION_STEP_SIZE_FRACTION_OF_SQUARE: f64 = 9.0/10.0;
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
        width: usize,
        height: usize,
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
        output_file: String,
        show_progress: bool,
    ) -> Self {
        Self {
            start_x,
            start_y,
            start_dir_x,
            start_dir_y,
            start_angle_deg,
            step_size,
            radius,
            width,
            height,
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
            output_file,
            show_progress,
        }
    }

    pub fn init(args: &crate::args::Args) -> Self {
        Self::new(
            args.start_x,
            args.start_y,
            args.dir_x,
            args.dir_y,
            0.0,
            args.square_size * SIMULATION_STEP_SIZE_FRACTION_OF_SQUARE,
            args.radius,
            args.width,
            args.height,
            args.square_size,
            args.velocity,
            args.stop_coverage,
            args.stop_time,
            args.stop_bounces,
            args.stop_simsteps,
            args.stop_distance,
            args.parallel,
            args.image_width,
            args.image_height,
            args.output_file
                .clone()
                .unwrap_or_else(|| DEFAULT_IMAGE_FILE_NAME.to_string()),
            args.verbosity,
            args.track_center,
            args.output_file.clone().unwrap_or_else(|| DEFAULT_IMAGE_FILE_NAME.to_string()),
            args.show_progress,
        )
    }

    pub fn print_simulation_parameters(&self) {
        println!(
            "{}",
            "Simulation Parameters:".color(colored::Color::Blue).bold()
        );
        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Radius", self.radius
        );
        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Track Center", if self.track_center { "ENABLED" } else { "DISABLED" }
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
            self.width,
            self.height,
            self.width as f64 * self.square_size,
            self.height as f64 * self.square_size
        );
        println!("  {:<WIDTH_LABEL$}: {:.2}", "Square Size", self.square_size);
        println!(
            "  {:<WIDTH_LABEL$}: {:.2} units/s",
            "Velocity", self.velocity
        );
        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Perturbation",
            if self.perturb { "ENABLED" } else { "DISABLED" }
        );
        println!(
            "  {:<WIDTH_LABEL$}: {}",
            "Parallel processing",
            if self.parallel { "ENABLED" } else { "DISABLED" }
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
            "Simulation completed in",
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
            self.coverage_count,
            self.width * self.height
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
            "  {:<WIDTH_LABEL$}: {} (Using step size = {:.2} units)",
            "Total Simulation Steps",
            self.sim_steps.separate_with_commas(),
            self.step_size
        );
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
    // a) The radius must be larger than square/2, sq < 2r
    // b) The step size must be smaller tha n twice the radius, step_size < 2r
    if sim_model.radius < sim_model.square_size / 2.0 {
        return Err("Invalid radius or square size".into());
    }

    if sim_model.step_size > 2.0 * sim_model.radius {
        return Err("Invalid step size or radius".into());
    }

    // If both startx and start_y are zero, randomize the starting position
    if args.start_x == 0.0 && args.start_y == 0.0 {
        // Randomize start position within the bounding box
        sim_model.start_x = rng.random_range(
            sim_model.radius..(sim_model.width as f64 * sim_model.square_size - sim_model.radius),
        );
        sim_model.start_y = rng.random_range(
            sim_model.radius..(sim_model.height as f64 * sim_model.square_size - sim_model.radius),
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
