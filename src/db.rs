// use crate::model;
use crate::args;
use crate::db;
use crate::model::SimModel;
use colored::Colorize;
use rusqlite::{Connection, params};
use serde_json::Value as JsonValue;

pub struct Database {
    conn: Connection,
}

pub fn try_store_result_to_db(args: &args::Args, model: &SimModel) {
    // Store simulation data in database if requested
    if let Some(ref db_path) = args.database_file {
        match db::store_simulation_to_database(model, db_path) {
            Ok((model_id, result_id)) => {
                if !args.quiet {
                    let header = "Simulation data stored in database:";
                    println!(
                        "{}\n{}\n  Model ID: {}, Result ID: {} in '{}'",
                        header.color(colored::Color::Green).bold(),
                        "=".repeat(header.len()).color(colored::Color::Green).bold(),
                        model_id,
                        result_id,
                        db_path
                    );
                }
            }
            Err(err) => {
                eprintln!(
                    "{} {}",
                    "Error storing simulation data in database:"
                        .color(colored::Color::Red)
                        .bold(),
                    err
                );
            }
        }
    }
}

impl Database {
    /// Create a new database connection or open existing database
    pub fn new(db_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let conn = Connection::open(db_path)?;
        let mut db = Database { conn };
        db.create_tables()?;
        Ok(db)
    }

    /// Create the required tables if they don't exist
    fn create_tables(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Create models table
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS models (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            cutter_battery_charge_time REAL,
            cutter_battery_run_time REAL,
            cutter_blade_length REAL,
            cutter_radius REAL,
            cutter_radius_in_cells INTEGER,
            cutter_type TEXT,
            cutter_velocity REAL,
            cutter_wheel_inbalance_adjustment_step REAL,
            cutter_wheel_inbalance_enabled INTEGER,
            cutter_wheel_inbalance_radius REAL,
            cutter_wheel_inbalance_radius_max REAL,
            cutter_wheel_inbalance_radius_min REAL,
            cutter_wheel_slippage_activation_check_distance REAL,
            cutter_wheel_slippage_adjustment_step REAL,
            cutter_wheel_slippage_enabled INTEGER,
            cutter_wheel_slippage_max_duration_distance REAL,
            cutter_wheel_slippage_min_duration_distance REAL,
            cutter_wheel_slippage_probability REAL,
            cutter_wheel_slippage_radius_max REAL,
            cutter_wheel_slippage_radius_min REAL,
            frames_animation_file_name TEXT,
            frames_animation_speedup INTEGER,
            frames_create_animation INTEGER,
            frames_delete_frames INTEGER,
            frames_directory TEXT,
            frames_enabled INTEGER,
            frames_hw_encoding INTEGER,
            frames_rate_fps INTEGER,
            grid_cell_size REAL,
            grid_height_units REAL,
            grid_hor__cells INTEGER,
            grid_map_file_name TEXT,
            grid_obstacles_cells_with_obstacle INTEGER,
            grid_obstacles_num_obstacles INTEGER,
            grid_obstacles_percent REAL,
            grid_total_cells INTEGER,
            grid_ver__cells INTEGER,
            grid_width_units REAL,
            image_color_theme TEXT,
            image_dpi INTEGER,
            image_image_file_name TEXT,
            image_image_size_mm_height INTEGER,
            image_image_size_mm_width INTEGER,
            image_paper_size_format TEXT,
            image_paper_size_height_mm REAL,
            image_paper_size_width_mm REAL,
            image_show_gridlines INTEGER,
            simulation_perturb_segment INTEGER,
            simulation_perturb_segment_percent REAL,
            simulation_perturb_at_bounces INTEGER,
            simulation_quiet INTEGER,
            simulation_show_progress INTEGER,
            simulation_step_size REAL,
            simulation_track_center INTEGER,
            simulation_verbosity INTEGER,
            simulation_random_seed INTEGER,
            start_direction_angle_deg REAL,
            start_direction_dirx REAL,
            start_direction_diry REAL,
            start_position_x REAL,
            start_position_y REAL,
            stop_conditions_bounces INTEGER,
            stop_conditions_coverage_percent REAL,
            stop_conditions_distance_units REAL,
            stop_conditions_simulation_steps INTEGER,
            stop_conditions_time_seconds REAL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP)",
            [],
        )?;

        // Create results table with foreign key to models
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                model_id INTEGER NOT NULL,
                coverage_bounces INTEGER,
                coverage_cells INTEGER,
                coverage_max_visited INTEGER,
                coverage_min_visited INTEGER,
                coverage_percent REAL,
                cutter_battery_charge_count INTEGER,
                cutter_battery_charge_left_percent REAL,
                cutter_battery_charge_time REAL,
                cutter_battery_run_time REAL,
                cutter_blade_length REAL,
                cutter_cells_under REAL,
                cutter_distance REAL,
                cutter_radius REAL,
                cutter_radius_in_cells INTEGER,
                cutter_type TEXT,
                cutter_velocity REAL,
                cutter_wheel_inbalance INTEGER,
                cutter_wheel_slippage INTEGER,
                frames_animation INTEGER,
                frames_animation_speedup INTEGER,
                frames_animation_file_name TEXT,
                frames_delete_frames INTEGER,
                frames_directory TEXT,
                frames_enabled INTEGER,
                frames_hw_encoding INTEGER,
                frames_rate_fps INTEGER,
                frames_steps_per_frame INTEGER,
                grid_cell_side_units REAL,
                grid_height_units REAL,
                grid_hor_cells INTEGER,
                grid_obstacles_numcells INTEGER,
                grid_obstacles_percent REAL,
                grid_total_cells INTEGER,
                grid_vert_cells INTEGER,
                grid_width_units REAL,
                output_image_color_theme TEXT,
                output_image_dpi INTEGER,
                output_image_file_name TEXT,
                output_image_paper_size_format TEXT,
                output_image_paper_size_height_mm REAL,
                output_image_paper_size_width_mm REAL,
                output_image_pixels_height INTEGER,
                output_image_pixels_width INTEGER,
                output_image_show_gridlines INTEGER,
                start_angle_degrees REAL,
                start_direction_x REAL,
                start_direction_y REAL,
                start_position_x REAL,
                start_position_y REAL,
                steps_length_units REAL,
                steps_seconds_per_step REAL,
                steps_steps_per_cell REAL,
                steps_steps_per_second INTEGER,
                steps_total_num INTEGER,
                time_cpu_time TEXT,
                time_cpu_time_milliseconds INTEGER,
                time_cutting_time TEXT,
                time_cutting_time_seconds INTEGER,
                time_efficiency REAL,
                time_ffmpeg_encoding_duration TEXT,
                time_min_cov_time TEXT,
                time_min_cov_time_seconds INTEGER,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (model_id) REFERENCES models (id))",
            [],
        )?;

        Ok(())
    }

    /// Store both model and result in the database in a transaction
    pub fn store_simulation_data(
        &mut self,
        model: &SimModel,
    ) -> Result<(i64, i64), Box<dyn std::error::Error>> {
        let tx = self.conn.transaction()?;

        // Unfortunately we have to duplicate the store_model and store_result logic here
        // since we can only borrow self mutably once in a transaction.

        // Store model first
        let model_id = {
            let model_json = model.get_model_as_json();
            let model_data = &model_json["Model"];

            tx.execute(
                "INSERT INTO models (
                cutter_battery_charge_time,
                cutter_battery_run_time,
                cutter_blade_length,
                cutter_radius,
                cutter_radius_in_cells,
                cutter_type,
                cutter_velocity,
                cutter_wheel_inbalance_adjustment_step,
                cutter_wheel_inbalance_enabled,
                cutter_wheel_inbalance_radius,
                cutter_wheel_inbalance_radius_max,
                cutter_wheel_inbalance_radius_min,
                cutter_wheel_slippage_activation_check_distance,
                cutter_wheel_slippage_adjustment_step,
                cutter_wheel_slippage_enabled,
                cutter_wheel_slippage_max_duration_distance,
                cutter_wheel_slippage_min_duration_distance,
                cutter_wheel_slippage_probability,
                cutter_wheel_slippage_radius_max,
                cutter_wheel_slippage_radius_min,
                frames_animation_file_name,
                frames_animation_speedup,
                frames_create_animation,
                frames_delete_frames,
                frames_directory,
                frames_enabled,
                frames_hw_encoding,
                frames_rate_fps,
                grid_cell_size,
                grid_height_units,
                grid_hor__cells,
                grid_map_file_name,
                grid_obstacles_cells_with_obstacle,
                grid_obstacles_num_obstacles,
                grid_obstacles_percent,
                grid_total_cells,
                grid_ver__cells,
                grid_width_units,
                image_color_theme,
                image_dpi,
                image_image_file_name,
                image_image_size_mm_height,
                image_image_size_mm_width,
                image_paper_size_format,
                image_paper_size_height_mm,
                image_paper_size_width_mm,
                image_show_gridlines,
                simulation_perturb_segment,
                simulation_perturb_segment_percent,
                simulation_perturb_at_bounces,
                simulation_quiet,
                simulation_show_progress,
                simulation_step_size,
                simulation_track_center,
                simulation_verbosity,
                simulation_random_seed,
                start_direction_angle_deg,
                start_direction_dirx,
                start_direction_diry,
                start_position_x,
                start_position_y,
                stop_conditions_bounces,
                stop_conditions_coverage_percent,
                stop_conditions_distance_units,
                stop_conditions_simulation_steps,
                stop_conditions_time_seconds
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, 
                ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, ?40, 
                ?41, ?42, ?43, ?44, ?45, ?46, ?47, ?48, ?49, ?50, ?51, ?52, ?53, ?54, ?55, ?56, ?57, ?58, ?59, ?60, 
                ?61, ?62, ?63, ?64, ?65, ?66
            )",
            params![
                get_f64_from_json(&model_data["Cutter"]["Battery"]["Charge Time"]),
                get_f64_from_json(&model_data["Cutter"]["Battery"]["Run Time"]),
                get_f64_from_json(&model_data["Cutter"]["Blade Length"]),
                get_f64_from_json(&model_data["Cutter"]["Radius"]),
                get_i64_from_json(&model_data["Cutter"]["Radius in cells"]),
                get_string_from_json(&model_data["Cutter"]["Type"]),
                get_f64_from_json(&model_data["Cutter"]["Velocity"]),
                get_f64_from_json(&model_data["Cutter"]["Wheel Inbalance"]["Adjustment step"]),
                get_bool_as_i64_from_json(&model_data["Cutter"]["Wheel Inbalance"]["Enabled"]),
                get_f64_from_json(&model_data["Cutter"]["Wheel Inbalance"]["Radius"]),
                get_f64_from_json(&model_data["Cutter"]["Wheel Inbalance"]["Radius max"]),
                get_f64_from_json(&model_data["Cutter"]["Wheel Inbalance"]["Radius min"]),
                get_f64_from_json(&model_data["Cutter"]["Wheel Slippage"]["Activation check distance"]),
                get_f64_from_json(&model_data["Cutter"]["Wheel Slippage"]["Adjustment step"]),
                get_bool_as_i64_from_json(&model_data["Cutter"]["Wheel Slippage"]["Enabled"]),
                get_f64_from_json(&model_data["Cutter"]["Wheel Slippage"]["Max Duration Distance"]),
                get_f64_from_json(&model_data["Cutter"]["Wheel Slippage"]["Min Duration Distance"]),
                get_f64_from_json(&model_data["Cutter"]["Wheel Slippage"]["Probability"]),
                get_f64_from_json(&model_data["Cutter"]["Wheel Slippage"]["Radius max"]),
                get_f64_from_json(&model_data["Cutter"]["Wheel Slippage"]["Radius min"]),
                get_string_from_json(&model_data["Frames"]["Animation File Name"]),
                get_i64_from_json(&model_data["Frames"]["Animation Speedup"]),
                get_bool_as_i64_from_json(&model_data["Frames"]["Create Animation"]),
                get_bool_as_i64_from_json(&model_data["Frames"]["Delete Frames"]),
                get_string_from_json(&model_data["Frames"]["Directory"]),
                get_bool_as_i64_from_json(&model_data["Frames"]["Enabled"]),
                get_bool_as_i64_from_json(&model_data["Frames"]["HW Encoding"]),
                get_i64_from_json(&model_data["Frames"]["Rate (fps)"]),
                get_f64_from_json(&model_data["Grid"]["Cell Size"]),
                get_f64_from_json(&model_data["Grid"]["Height (units)"]),
                get_i64_from_json(&model_data["Grid"]["Hor. Cells"]),
                get_string_from_json(&model_data["Grid"]["Map File Name"]),
                get_i64_from_json(&model_data["Grid"]["Obstacles"]["Cells with obstacle"]),
                get_i64_from_json(&model_data["Grid"]["Obstacles"]["Num obstacles"]),
                get_f64_from_json(&model_data["Grid"]["Obstacles"]["Percent"]),
                get_i64_from_json(&model_data["Grid"]["Total Cells"]),
                get_i64_from_json(&model_data["Grid"]["Ver. Cells"]),
                get_f64_from_json(&model_data["Grid"]["Width (units)"]),
                get_string_from_json(&model_data["Image"]["Color Theme"]),
                get_i64_from_json(&model_data["Image"]["DPI"]),
                get_string_from_json(&model_data["Image"]["Image File Name"]),
                get_i64_from_json(&model_data["Image"]["Image Size (mm)"]["Height"]),
                get_i64_from_json(&model_data["Image"]["Image Size (mm)"]["Width"]),
                get_string_from_json(&model_data["Image"]["Paper Size"]["format"]),
                get_f64_from_json(&model_data["Image"]["Paper Size"]["height_mm"]),
                get_f64_from_json(&model_data["Image"]["Paper Size"]["width_mm"]),
                get_bool_as_i64_from_json(&model_data["Image"]["Show Gridlines"]),
                get_bool_as_i64_from_json(&model_data["Simulation"]["Perturb Segment"]),
                get_f64_from_json(&model_data["Simulation"]["Perturb Segment Percent"]),
                get_bool_as_i64_from_json(&model_data["Simulation"]["Perturb at Bounces"]),
                get_bool_as_i64_from_json(&model_data["Simulation"]["Quiet"]),
                get_bool_as_i64_from_json(&model_data["Simulation"]["Show Progress"]),
                get_f64_from_json(&model_data["Simulation"]["Step Size"]),
                get_bool_as_i64_from_json(&model_data["Simulation"]["Track Center"]),
                get_i64_from_json(&model_data["Simulation"]["Verbosity"]),
                get_i64_from_json(&model_data["Simulation"]["Random Seed"]),
                get_f64_from_json(&model_data["Start"]["Direction"]["Angle (deg)"]),
                get_f64_from_json(&model_data["Start"]["Direction"]["DirX"]),
                get_f64_from_json(&model_data["Start"]["Direction"]["DirY"]),
                get_f64_from_json(&model_data["Start"]["Position"]["X"]),
                get_f64_from_json(&model_data["Start"]["Position"]["Y"]),
                get_i64_from_json(&model_data["Stop Conditions"]["Bounces"]),
                get_f64_from_json(&model_data["Stop Conditions"]["Coverage (%)"]),
                get_f64_from_json(&model_data["Stop Conditions"]["Distance (units)"]),
                get_i64_from_json(&model_data["Stop Conditions"]["Simulation Steps"]),
                get_f64_from_json(&model_data["Stop Conditions"]["Time (seconds)"])
            ])?;
            tx.last_insert_rowid()
        };

        // Store result with foreign key reference
        let result_id = {
            let result_json = model.get_simulation_result_as_json();
            let result_data = &result_json["Result"];

            tx.execute(
                "INSERT INTO results (
                    model_id,
                    coverage_bounces,
                    coverage_cells,
                    coverage_max_visited,
                    coverage_min_visited,
                    coverage_percent,
                    cutter_battery_charge_count,
                    cutter_battery_charge_left_percent,
                    cutter_battery_charge_time,
                    cutter_battery_run_time,
                    cutter_blade_length,
                    cutter_cells_under,
                    cutter_distance,
                    cutter_radius,
                    cutter_radius_in_cells,
                    cutter_type,
                    cutter_velocity,
                    cutter_wheel_inbalance,
                    cutter_wheel_slippage,
                    frames_animation,
                    frames_animation_speedup,
                    frames_animation_file_name,
                    frames_delete_frames,
                    frames_directory,
                    frames_enabled,
                    frames_hw_encoding,
                    frames_rate_fps,
                    frames_steps_per_frame,
                    grid_cell_side_units,
                    grid_height_units,
                    grid_hor_cells,
                    grid_obstacles_numcells,
                    grid_obstacles_percent,
                    grid_total_cells,
                    grid_vert_cells,
                    grid_width_units,
                    output_image_color_theme,
                    output_image_dpi,
                    output_image_file_name,
                    output_image_paper_size_format,
                    output_image_paper_size_height_mm,
                    output_image_paper_size_width_mm,
                    output_image_pixels_height,
                    output_image_pixels_width,
                    output_image_show_gridlines,
                    start_angle_degrees,
                    start_direction_x,
                    start_direction_y,
                    start_position_x,
                    start_position_y,
                    steps_length_units,
                    steps_seconds_per_step,
                    steps_steps_per_cell,
                    steps_steps_per_second,
                    steps_total_num,
                    time_cpu_time,
                    time_cpu_time_milliseconds,
                    time_cutting_time,
                    time_cutting_time_seconds,
                    time_efficiency,
                    time_ffmpeg_encoding_duration,
                    time_min_cov_time,
                    time_min_cov_time_seconds
                ) VALUES (
                    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, 
                    ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38, ?39, 
                    ?40, ?41, ?42, ?43, ?44, ?45, ?46, ?47, ?48, ?49, ?50, ?51, ?52, ?53, ?54, ?55, ?56, ?57, ?58, 
                    ?59, ?60, ?61, ?62, ?63
                )",
                params![
                    model_id,
                    get_i64_from_json(&result_data["Coverage"]["Bounces"]),
                    get_i64_from_json(&result_data["Coverage"]["Cells"]),
                    get_i64_from_json(&result_data["Coverage"]["Max visited"]),
                    get_i64_from_json(&result_data["Coverage"]["Min visited"]),
                    get_f64_from_json(&result_data["Coverage"]["Percent"]),
                    get_i64_from_json(&result_data["Cutter"]["Battery"]["Charge count"]),
                    get_f64_from_json(&result_data["Cutter"]["Battery"]["Charge left (%)"]),
                    get_f64_from_json(&result_data["Cutter"]["Battery"]["Charge time"]),
                    get_f64_from_json(&result_data["Cutter"]["Battery"]["Run time"]),
                    get_f64_from_json(&result_data["Cutter"]["Blade Length"]),
                    get_f64_from_json(&result_data["Cutter"]["Cells under"]),
                    get_f64_from_json(&result_data["Cutter"]["Distance"]),
                    get_f64_from_json(&result_data["Cutter"]["Radius"]),
                    get_i64_from_json(&result_data["Cutter"]["Radius in cells"]),
                    get_string_from_json(&result_data["Cutter"]["Type"]),
                    get_f64_from_json(&result_data["Cutter"]["Velocity"]),
                    get_bool_as_i64_from_json(&result_data["Cutter"]["Wheel Inbalance"]),
                    get_bool_as_i64_from_json(&result_data["Cutter"]["Wheel Slippage"]),
                    get_bool_as_i64_from_json(&result_data["Frames"]["Animation"]),
                    get_i64_from_json(&result_data["Frames"]["Animation Speedup"]),
                    get_string_from_json(&result_data["Frames"]["Animation file name"]),
                    get_bool_as_i64_from_json(&result_data["Frames"]["Delete frames"]),
                    get_string_from_json(&result_data["Frames"]["Directory"]),
                    get_bool_as_i64_from_json(&result_data["Frames"]["Enabled"]),
                    get_bool_as_i64_from_json(&result_data["Frames"]["HW Encoding"]),
                    get_i64_from_json(&result_data["Frames"]["Rate (fps)"]),
                    get_i64_from_json(&result_data["Frames"]["Steps per frame"]),
                    get_f64_from_json(&result_data["Grid"]["Cell side (units)"]),
                    get_f64_from_json(&result_data["Grid"]["Height (units)"]),
                    get_i64_from_json(&result_data["Grid"]["Hor.Cells"]),
                    get_i64_from_json(&result_data["Grid"]["Obstacles"]["NumCells"]),
                    get_f64_from_json(&result_data["Grid"]["Obstacles"]["Percent"]),
                    get_i64_from_json(&result_data["Grid"]["Total cells"]),
                    get_i64_from_json(&result_data["Grid"]["Vert.Cells"]),
                    get_f64_from_json(&result_data["Grid"]["Width (units)"]),
                    get_string_from_json(&result_data["Output image"]["Color Theme"]),
                    get_i64_from_json(&result_data["Output image"]["DPI"]),
                    get_string_from_json(&result_data["Output image"]["File name"]),
                    get_string_from_json(&result_data["Output image"]["Paper size"]["format"]),
                    get_f64_from_json(&result_data["Output image"]["Paper size"]["height_mm"]),
                    get_f64_from_json(&result_data["Output image"]["Paper size"]["width_mm"]),
                    get_i64_from_json(&result_data["Output image"]["Pixels"]["height"]),
                    get_i64_from_json(&result_data["Output image"]["Pixels"]["width"]),
                    get_bool_as_i64_from_json(&result_data["Output image"]["Show gridlines"]),
                    get_f64_from_json(&result_data["Start"]["Angle (degrees)"]),
                    get_f64_from_json(&result_data["Start"]["Direction"]["X"]),
                    get_f64_from_json(&result_data["Start"]["Direction"]["Y"]),
                    get_f64_from_json(&result_data["Start"]["Position"]["X"]),
                    get_f64_from_json(&result_data["Start"]["Position"]["Y"]),
                    get_f64_from_json(&result_data["Steps"]["Length (units)"]),
                    get_f64_from_json(&result_data["Steps"]["Seconds/step"]),
                    get_f64_from_json(&result_data["Steps"]["Steps/cell"]),
                    get_i64_from_json(&result_data["Steps"]["Steps/second"]),
                    get_i64_from_json(&result_data["Steps"]["Total #"]),
                    get_string_from_json(&result_data["Time"]["CPU time"]),
                    get_i64_from_json(&result_data["Time"]["CPU time (milliseconds)"]),
                    get_string_from_json(&result_data["Time"]["Cutting time"]),
                    get_i64_from_json(&result_data["Time"]["Cutting time (seconds)"]),
                    get_f64_from_json(&result_data["Time"]["Efficiency"]),
                    get_string_from_json(&result_data["Time"]["FFmpeg Encoding Duration"]),
                    get_string_from_json(&result_data["Time"]["Min.Cov.Time"]),
                    get_i64_from_json(&result_data["Time"]["Min.Cov.Time (seconds)"])
                ],
            )?;

            tx.last_insert_rowid()
        };
        tx.commit()?;
        Ok((model_id, result_id))
    }
}

/// Utility function to store both model and results data from a simulation
pub fn store_simulation_to_database(
    model: &SimModel,
    db_path: &str,
) -> Result<(i64, i64), Box<dyn std::error::Error>> {
    let mut db = Database::new(db_path)?;
    db.store_simulation_data(model)
}

// Helper functions to extract values from JSON with proper type conversion
fn get_string_from_json(value: &JsonValue) -> String {
    match value {
        JsonValue::String(s) => s.clone(),
        JsonValue::Null => "".to_string(),
        _ => value.to_string().trim_matches('"').to_string(),
    }
}

fn get_f64_from_json(value: &JsonValue) -> f64 {
    match value {
        JsonValue::Number(n) => n.as_f64().unwrap_or(0.0),
        JsonValue::String(s) => s.parse::<f64>().unwrap_or(0.0),
        _ => 0.0,
    }
}

fn get_i64_from_json(value: &JsonValue) -> i64 {
    match value {
        JsonValue::Number(n) => n.as_i64().unwrap_or(0),
        JsonValue::String(s) => s.parse::<i64>().unwrap_or(0),
        _ => 0,
    }
}

fn get_bool_as_i64_from_json(value: &JsonValue) -> i64 {
    match value {
        JsonValue::Bool(b) => {
            if *b {
                1
            } else {
                0
            }
        }
        JsonValue::Number(n) => {
            if n.as_i64().unwrap_or(0) != 0 {
                1
            } else {
                0
            }
        }
        JsonValue::String(s) => {
            if s.to_lowercase() == "true" {
                1
            } else {
                0
            }
        }
        _ => 0,
    }
}
