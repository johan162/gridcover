use crate::args;
use chrono::Duration;
use colored::Colorize;
use rand::Rng;
use serde_json::json;
use thousands::Separable;

const MIN_RADIUS: f64 = 0.05;
const MIN_BLADE_LEN: f64 = 0.01;

pub mod papersize;
pub mod boundingbox;
pub mod coverageinfo;
pub mod cuttertype;

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
    pub grid_width: f64,  // Width of the grid in pixels
    pub grid_height: f64, // Height of the grid in pixels
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
    pub image_width_mm: u32,
    pub image_height_mm: u32,
    pub image_file_name: Option<String>,
    pub verbosity: usize,
    pub track_center: bool,
    pub show_progress: bool,
    pub blade_len: f64,
    pub cutter_type: cuttertype::CutterType,
    pub dpi: u32,
    pub perturb_segment: bool,
    pub perturb_segment_percent: f64,
    pub coverage_grid: Vec<Vec<coverageinfo::CoverageInfo>>,
    pub bb: boundingbox::BoundingBox,
    pub battery_run_time: f64,
    pub battery_charge_time: f64,
    pub battery_charge_count: usize,
    pub battery_charge_left: f64, // Percentage of battery charge left
    pub paper_size: papersize::PaperSize,
}

// Define a constant for the simulation step size factor
// This factor determines how many simulation steps are taken per square (cell) size
// To get a full coverage of squares we should always have a step size that is a fraction of the square size.
const SIMULATION_STEP_SIZE_FRACTION_OF_SQUARE: f64 = 3.0 / 5.0;
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
        grid_width: f64,
        grid_height: f64,
        square_size: f64,
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
            coverage_grid: vec![vec![coverageinfo::CoverageInfo::new(); 1]; 1], // Placeholder, will be initialized later when we now how many cells are needed
            bb: boundingbox::BoundingBox::init(grid_width, grid_height, radius), // TODO: This might not be set at this staeg!
            battery_run_time,
            battery_charge_time,
            battery_charge_count: 0,
            battery_charge_left: 100.0,
            paper_size,
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
        )
    }

    pub fn print_simulation_parameters(&self) {
        println!(
            "{}",
            "Simulation Parameters:".color(colored::Color::Blue).bold()
        );
        println!("  {:<WIDTH_LABEL$}: {:.4}", "Radius", self.radius);
        println!(
            "  {:<WIDTH_LABEL$}: {:.4} units",
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
        println!("  {:<WIDTH_LABEL$}: {:.4}", "Step Size", self.step_size);
        println!("  {:<WIDTH_LABEL$}: {:.4}", "Radius", self.radius);
        println!(
            "  {:<WIDTH_LABEL$}: {}x{} squares (={}x{} units)",
            "Grid Size",
            self.grid_cells_x,
            self.grid_cells_y,
            self.grid_cells_x as f64 * self.square_size,
            self.grid_cells_y as f64 * self.square_size
        );
        println!("  {:<WIDTH_LABEL$}: {:.4}", "Square Size", self.square_size);
        println!(
            "  {:<WIDTH_LABEL$}: {:.4} units/s",
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
            if self.perturb_segment {
                "ENABLED"
            } else {
                "DISABLED"
            }
        );
        println!(
            "  {:<WIDTH_LABEL$}: {:.1}%",
            "Perturb Segment Percent",
            self.perturb_segment_percent * 100.0
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
            "  {:<WIDTH_LABEL$}: {:?}",
            "Image File Name", self.image_file_name
        );

        println!("  {:<WIDTH_LABEL$}: {} dpi", "Image DPI", self.dpi);

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

        let theoretical_minimum_time_seconds = (self.grid_height / self.velocity
            * (self.grid_width / (self.radius * 2.0)).ceil()
            * (self.coverage_percent / 100.0))
            .ceil() as u64;
        let t_hours = theoretical_minimum_time_seconds / 3600;
        let t_minutes = (theoretical_minimum_time_seconds % 3600) / 60;
        let t_seconds = theoretical_minimum_time_seconds % 60;
        println!(
            "  {:<WIDTH_LABEL$}: {t_hours}:{t_minutes:02}:{t_seconds:02}",
            "Theoretical minimum time"
        );
        println!(
            "  {:<WIDTH_LABEL$}: {:.1}% ({} out of {} cells)",
            "Coverage",
            self.coverage_percent,
            self.coverage_count.separate_with_commas(),
            (self.grid_cells_x * self.grid_cells_y).separate_with_commas()
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
            "  {:<WIDTH_LABEL$}: {:.1}%",
            "Battery Charge Left", self.battery_charge_left
        );
        println!(
            "  {:<WIDTH_LABEL$}: {} (Step size = {:.2} units, Sim steps/cell = {:.2})",
            "Total Simulation Steps",
            self.sim_steps.separate_with_commas(),
            self.step_size,
            self.step_size / self.square_size
        );
    }

    /// A version of print_simulation_results() that outputs results in JSON format
    pub fn print_simulation_results_json(&self) {
        let total_seconds = self.sim_time_elapsed as u64;
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        let theoretical_minimum_time_seconds = (self.grid_height / self.velocity
            * (self.grid_width / (self.radius * 2.0)).ceil()
            * (self.coverage_percent / 100.0))
            .ceil() as u64;
        let t_hours = theoretical_minimum_time_seconds / 3600;
        let t_minutes = (theoretical_minimum_time_seconds % 3600) / 60;
        let t_seconds = theoretical_minimum_time_seconds % 60;

        // Algorithm efficience defined as percentage in relation to how close we get to
        // the theoretical minimum time. An efficiency of 100% means we reached the theoretical minimum time.
        let efficiency = if theoretical_minimum_time_seconds > 0 {
            (theoretical_minimum_time_seconds as f64 / self.sim_time_elapsed) * 100.0
        } else {
            0.0
        };

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
                        "Charge left (percent)": self.battery_charge_left,
                    }
                },
                "Time": {
                     "Real": format!("{:02}:{:02}",
                        self.sim_real_time.num_minutes(),
                        self.sim_real_time.num_seconds() % 60),
                    "Simulation": format!("{:02}:{:02}:{:02}",
                        hours, minutes, seconds),
                    "Min. Time": format!("{:02}:{:02}:{:02}",
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
                    "Cells x": self.grid_cells_x,
                    "Cells y": self.grid_cells_y,
                    "Area": self.grid_cells_x * self.grid_cells_y,
                    "Cell size": self.square_size,
                    "Width": self.square_size * self.grid_cells_x as f64,
                    "Height": self.square_size * self.grid_cells_y as f64,
                },
                "Steps": {
                    "Total": self.sim_steps,
                    "Size": self.step_size,
                    "Per cell": self.step_size / self.square_size,
                },
                "Output image": {
                    "Paper size": self.paper_size.get_json(),
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

    if model.square_size <= 0.0 {
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
            model.square_size = model.blade_len / 2.0; // Default square size for blade cutter
        } else {
            model.square_size = model.radius / 3.0; // Default square size for circular cutter
        }
    }

    if model.grid_width == 0.0 && model.grid_height == 0.0 {
        // Set default grid size based on cutter radious size
        model.grid_width = model.radius * 50.0;
        model.grid_height = model.radius * 50.0;
    } else if model.grid_width == 0.0 {
        model.grid_width = model.grid_height; // Default grid width
    } else {
        model.grid_height = model.grid_width; // Default grid height
    }

    if model.grid_width < model.radius * 4.0 || model.grid_height < model.radius * 4.0 {
        return Err(format!(
            "Grid size must be at least 4 times the radius (Minimum {}x{} units)",
            4.0 * model.radius,
            4.0 * model.radius
        )
        .into());
    }

    // Calculate the number of cells in the grid based on the square size
    // Round the number of cells to the nearest whole number
    model.grid_cells_x = (model.grid_width / model.square_size).round() as usize;
    model.grid_cells_y = (model.grid_height / model.square_size).round() as usize;

    model.coverage_grid = vec![vec![coverageinfo::CoverageInfo::new(); model.grid_cells_y]; model.grid_cells_x];

    // Re-adjust the width/height to make sure it is a whole number of cells
    model.grid_width = model.grid_cells_x as f64 * model.square_size;
    model.grid_height = model.grid_cells_y as f64 * model.square_size;

    model.bb = boundingbox::BoundingBox::init(model.grid_width, model.grid_height, model.radius);

    if model.cutter_type == cuttertype::CutterType::Blade && (model.square_size > model.blade_len / 1.5) {
        return Err("Square size must be < 3/2 of blade length".into());
    }

    if model.step_size >= model.square_size {
        return Err(format!(
            "Step size {} must be smaller than square size {}",
            model.step_size, model.square_size
        )
        .into());
    }

    // The size of the simulated grid must be at leat twice the radius
    if model.grid_cells_x as f64 * model.square_size <= 2.0 * model.radius
        || model.grid_cells_y as f64 * model.square_size <= 2.0 * model.radius
    {
        return Err(format!(
            "Grid size must be at least twice the diameter of the circle (Minimum {}x{} squares)",
            2 * (model.radius / model.square_size).ceil() as usize,
            2 * (model.radius / model.square_size).ceil() as usize
        )
        .into());
    }

    // If step size is not set then set it to 80% of the square size
    if model.step_size <= 0.0 {
        model.step_size = model.square_size * SIMULATION_STEP_SIZE_FRACTION_OF_SQUARE;
    }

    // If both startx and start_y are zero, randomize the starting position
    if args.start_x == 0.0 && args.start_y == 0.0 {
        model.start_x = rng.random_range(
            model.radius..(model.grid_cells_x as f64 * model.square_size - model.radius),
        );
        model.start_y = rng.random_range(
            model.radius..(model.grid_cells_y as f64 * model.square_size - model.radius),
        );
    } else {
        // Use the user-defined start position
        model.start_x = args.start_x;
        model.start_y = args.start_y;
    }

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
