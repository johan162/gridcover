use crate::model::SimModel;
use rusqlite::{Connection, params};
use serde_json::Value as JsonValue;

pub struct Database {
    conn: Connection,
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
                cutter_type TEXT,
                cutter_blade_length REAL,
                cutter_radius REAL,
                cutter_battery_run_time REAL,
                cutter_battery_charge_time REAL,
                cutter_velocity REAL,
                simulation_verbosity INTEGER,
                simulation_show_progress INTEGER,
                simulation_track_center INTEGER,
                simulation_step_size REAL,
                simulation_perturb_at_bounces INTEGER,
                simulation_perturb_segment INTEGER,
                simulation_perturb_segment_percent REAL,
                simulation_parallel_processing INTEGER,
                start_position_x REAL,
                start_position_y REAL,
                start_direction_dirx REAL,
                start_direction_diry REAL,
                start_direction_angle_deg REAL,
                grid_hor_cells INTEGER,
                grid_ver_cells INTEGER,
                grid_total_cells INTEGER,
                grid_width_units REAL,
                grid_height_units REAL,
                grid_cell_size REAL,
                grid_map_file_name TEXT,
                grid_obstacles_num_obstacles INTEGER,
                grid_obstacles_cells_with_obstacle INTEGER,
                grid_obstacles_percent REAL,
                image_image_size_width INTEGER,
                image_image_size_height INTEGER,
                image_image_file_name TEXT,
                image_dpi INTEGER,
                image_paper_size_name TEXT,
                image_paper_size_width_mm REAL,
                image_paper_size_height_mm REAL,
                image_show_gridlines INTEGER,
                stop_conditions_bounces INTEGER,
                stop_conditions_time_seconds REAL,
                stop_conditions_coverage_percent REAL,
                stop_conditions_simulation_steps INTEGER,
                stop_conditions_distance_units REAL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        )?;

        // Create results table with foreign key to models
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS results (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                model_id INTEGER NOT NULL,
                coverage_percent REAL,
                coverage_cells INTEGER,
                coverage_bounces INTEGER,
                coverage_max_visited INTEGER,
                coverage_min_visited INTEGER,
                cutter_type TEXT,
                cutter_blade_length REAL,
                cutter_radius REAL,
                cutter_velocity REAL,
                cutter_distance REAL,
                cutter_cells_covered REAL,
                cutter_battery_run_time REAL,
                cutter_battery_charge_time REAL,
                cutter_battery_charge_count INTEGER,
                cutter_battery_charge_left_percent REAL,
                time_cpu_time TEXT,
                time_cutting_time TEXT,
                time_min_cov_time TEXT,
                time_efficiency REAL,
                start_position_x REAL,
                start_position_y REAL,
                start_direction_x REAL,
                start_direction_y REAL,
                start_angle_degrees REAL,
                grid_hor_cells INTEGER,
                grid_vert_cells INTEGER,
                grid_total_cells INTEGER,
                grid_obstacles_num_cells INTEGER,
                grid_obstacles_percent REAL,
                grid_cell_side_units REAL,
                grid_width_units REAL,
                grid_height_units REAL,
                steps_total INTEGER,
                steps_size_units REAL,
                steps_per_cell REAL,
                output_image_paper_size_name TEXT,
                output_image_paper_size_width_mm REAL,
                output_image_paper_size_height_mm REAL,
                output_image_show_gridlines INTEGER,
                output_image_file_name TEXT,
                output_image_dpi INTEGER,
                output_image_pixels_width INTEGER,
                output_image_pixels_height INTEGER,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (model_id) REFERENCES models (id)
            )",
            [],
        )?;

        Ok(())
    }

    /// Store model parameters in the database and return the inserted model ID
    #[allow(dead_code)]
    pub fn store_model(&mut self, model: &SimModel) -> Result<i64, Box<dyn std::error::Error>> {
        let model_json = model.get_model_as_json();
        let model_data = &model_json["Model"];

        let _model_id = self.conn.execute(
            "INSERT INTO models (
                cutter_type, cutter_blade_length, cutter_radius, cutter_battery_run_time,
                cutter_battery_charge_time, cutter_velocity, simulation_verbosity,
                simulation_show_progress, simulation_track_center, simulation_step_size,
                simulation_perturb_at_bounces, simulation_perturb_segment,
                simulation_perturb_segment_percent, simulation_parallel_processing,
                start_position_x, start_position_y, start_direction_dirx,
                start_direction_diry, start_direction_angle_deg, grid_hor_cells,
                grid_ver_cells, grid_total_cells, grid_width_units, grid_height_units,
                grid_cell_size, grid_map_file_name, grid_obstacles_num_obstacles,
                grid_obstacles_cells_with_obstacle, grid_obstacles_percent,
                image_image_size_width, image_image_size_height, image_image_file_name,
                image_dpi, image_paper_size_name, image_paper_size_width_mm,
                image_paper_size_height_mm, image_show_gridlines, stop_conditions_bounces,
                stop_conditions_time_seconds, stop_conditions_coverage_percent,
                stop_conditions_simulation_steps, stop_conditions_distance_units
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38,
                ?39, ?40, ?41, ?42
            )",
            params![
                // Cutter
                get_string_from_json(&model_data["Cutter"]["Type"]),
                get_f64_from_json(&model_data["Cutter"]["Blade Length"]),
                get_f64_from_json(&model_data["Cutter"]["Radius"]),
                get_f64_from_json(&model_data["Cutter"]["Battery"]["Run Time"]),
                get_f64_from_json(&model_data["Cutter"]["Battery"]["Charge Time"]),
                get_f64_from_json(&model_data["Cutter"]["Velocity"]),
                // Simulation
                get_i64_from_json(&model_data["Simulation"]["Verbosity"]),
                get_bool_as_i64_from_json(&model_data["Simulation"]["Show Progress"]),
                get_bool_as_i64_from_json(&model_data["Simulation"]["Track Center"]),
                get_f64_from_json(&model_data["Simulation"]["Step Size"]),
                get_bool_as_i64_from_json(&model_data["Simulation"]["Perturb at Bounces"]),
                get_bool_as_i64_from_json(&model_data["Simulation"]["Perturb Segment"]),
                get_f64_from_json(&model_data["Simulation"]["Perturb Segment Percent"]),
                get_bool_as_i64_from_json(&model_data["Simulation"]["Parallel Processing"]),
                // Start
                get_f64_from_json(&model_data["Start"]["Position"]["X"]),
                get_f64_from_json(&model_data["Start"]["Position"]["Y"]),
                get_f64_from_json(&model_data["Start"]["Direction"]["DirX"]),
                get_f64_from_json(&model_data["Start"]["Direction"]["DirY"]),
                get_f64_from_json(&model_data["Start"]["Direction"]["Angle (deg)"]),
                // Grid
                get_i64_from_json(&model_data["Grid"]["Hor. Cells"]),
                get_i64_from_json(&model_data["Grid"]["Ver. Cells"]),
                get_i64_from_json(&model_data["Grid"]["Total Cells"]),
                get_f64_from_json(&model_data["Grid"]["Width (units)"]),
                get_f64_from_json(&model_data["Grid"]["Height (units)"]),
                get_f64_from_json(&model_data["Grid"]["Cell Size"]),
                get_string_from_json(&model_data["Grid"]["Map File Name"]),
                get_i64_from_json(&model_data["Grid"]["Obstacles"]["Num obstacles"]),
                get_i64_from_json(&model_data["Grid"]["Obstacles"]["Cells with obstacle"]),
                get_f64_from_json(&model_data["Grid"]["Obstacles"]["Percent"]),
                // Image
                get_i64_from_json(&model_data["Image"]["Image Size (mm)"]["Width"]),
                get_i64_from_json(&model_data["Image"]["Image Size (mm)"]["Height"]),
                get_string_from_json(&model_data["Image"]["Image File Name"]),
                get_i64_from_json(&model_data["Image"]["DPI"]),
                get_string_from_json(&model_data["Image"]["Paper Size"]["name"]),
                get_f64_from_json(&model_data["Image"]["Paper Size"]["width_mm"]),
                get_f64_from_json(&model_data["Image"]["Paper Size"]["height_mm"]),
                get_bool_as_i64_from_json(&model_data["Image"]["Show Gridlines"]),
                // Stop Conditions
                get_i64_from_json(&model_data["Stop Conditions"]["Bounces"]),
                get_f64_from_json(&model_data["Stop Conditions"]["Time (seconds)"]),
                get_f64_from_json(&model_data["Stop Conditions"]["Coverage (%)"]),
                get_i64_from_json(&model_data["Stop Conditions"]["Simulation Steps"]),
                get_f64_from_json(&model_data["Stop Conditions"]["Distance (units)"]),
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Store simulation results in the database
    #[allow(dead_code)]
    pub fn store_result(&mut self, model: &SimModel, model_id: i64) -> Result<i64, Box<dyn std::error::Error>> {
        let result_json = model.get_simulation_result_as_json();
        let result_data = &result_json["Simulation Result"];

        let _result_id = self.conn.execute(
            "INSERT INTO results (
                model_id, coverage_percent, coverage_cells, coverage_bounces,
                coverage_max_visited, coverage_min_visited, cutter_type,
                cutter_blade_length, cutter_radius, cutter_velocity, cutter_distance,
                cutter_cells_covered, cutter_battery_run_time, cutter_battery_charge_time,
                cutter_battery_charge_count, cutter_battery_charge_left_percent,
                time_cpu_time, time_cutting_time, time_min_cov_time, time_efficiency,
                start_position_x REAL,
                start_position_y REAL,
                start_direction_x REAL,
                start_direction_y REAL,
                start_angle_degrees REAL,
                grid_hor_cells INTEGER,
                grid_vert_cells INTEGER,
                grid_total_cells INTEGER,
                grid_obstacles_num_cells INTEGER,
                grid_obstacles_percent REAL,
                grid_cell_side_units REAL,
                grid_width_units REAL,
                grid_height_units REAL,
                steps_total INTEGER,
                steps_size_units REAL,
                steps_per_cell REAL,
                output_image_paper_size_name TEXT,
                output_image_paper_size_width_mm REAL,
                output_image_paper_size_height_mm REAL,
                output_image_show_gridlines INTEGER,
                output_image_file_name TEXT,
                output_image_dpi INTEGER,
                output_image_pixels_width INTEGER,
                output_image_pixels_height INTEGER,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38,
                ?39, ?40, ?41, ?42, ?43, ?44
            )",
            params![
                model_id,
                // Coverage
                get_f64_from_json(&result_data["Coverage"]["Percent"]),
                get_i64_from_json(&result_data["Coverage"]["Cells"]),
                get_i64_from_json(&result_data["Coverage"]["Bounces"]),
                get_i64_from_json(&result_data["Coverage"]["Max visited"]),
                get_i64_from_json(&result_data["Coverage"]["Min visited"]),
                // Cutter
                get_string_from_json(&result_data["Cutter"]["Type"]),
                get_f64_from_json(&result_data["Cutter"]["Blade Length"]),
                get_f64_from_json(&result_data["Cutter"]["Radius"]),
                get_f64_from_json(&result_data["Cutter"]["Velocity"]),
                get_f64_from_json(&result_data["Cutter"]["Distance"]),
                get_f64_from_json(&result_data["Cutter"]["Cells covered"]),
                get_f64_from_json(&result_data["Cutter"]["Battery"]["Run time"]),
                get_f64_from_json(&result_data["Cutter"]["Battery"]["Charge time"]),
                get_i64_from_json(&result_data["Cutter"]["Battery"]["Charge count"]),
                get_f64_from_json(&result_data["Cutter"]["Battery"]["Charge left (%)"]),
                // Time
                get_string_from_json(&result_data["Time"]["CPU time"]),
                get_string_from_json(&result_data["Time"]["Cutting time"]),
                get_string_from_json(&result_data["Time"]["Min.Cov.Time"]),
                get_f64_from_json(&result_data["Time"]["Efficiency"]),
                // Start
                get_f64_from_json(&result_data["Start"]["Position"]["X"]),
                get_f64_from_json(&result_data["Start"]["Position"]["Y"]),
                get_f64_from_json(&result_data["Start"]["Direction"]["X"]),
                get_f64_from_json(&result_data["Start"]["Direction"]["Y"]),
                get_f64_from_json(&result_data["Start"]["Angle (degrees)"]),
                // Grid
                get_i64_from_json(&result_data["Grid"]["Hor.Cells"]),
                get_i64_from_json(&result_data["Grid"]["Vert.Cells"]),
                get_i64_from_json(&result_data["Grid"]["Total cells"]),
                get_i64_from_json(&result_data["Grid"]["Obstacles"]["NumCells"]),
                get_f64_from_json(&result_data["Grid"]["Obstacles"]["Percent"]),
                get_f64_from_json(&result_data["Grid"]["Cell side (units)"]),
                get_f64_from_json(&result_data["Grid"]["Width (units)"]),
                get_f64_from_json(&result_data["Grid"]["Height (units)"]),
                // Steps
                get_i64_from_json(&result_data["Steps"]["Total"]),
                get_f64_from_json(&result_data["Steps"]["Size (units)"]),
                get_f64_from_json(&result_data["Steps"]["Per cell"]),
                // Output image
                get_string_from_json(&result_data["Output image"]["Paper size"]["name"]),
                get_f64_from_json(&result_data["Output image"]["Paper size"]["width_mm"]),
                get_f64_from_json(&result_data["Output image"]["Paper size"]["height_mm"]),
                get_bool_as_i64_from_json(&result_data["Output image"]["Show gridlines"]),
                get_string_from_json(&result_data["Output image"]["File name"]),
                get_i64_from_json(&result_data["Output image"]["DPI"]),
                get_i64_from_json(&result_data["Output image"]["Pixels"]["width"]),
                get_i64_from_json(&result_data["Output image"]["Pixels"]["height"]),
            ],
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Store both model and result in the database in a transaction
    pub fn store_simulation_data(&mut self, model: &SimModel) -> Result<(i64, i64), Box<dyn std::error::Error>> {
        let tx = self.conn.transaction()?;
        
        // Store model first
        let model_id = {
            let model_json = model.get_model_as_json();
            let model_data = &model_json["Model"];

            tx.execute(
                "INSERT INTO models (
                    cutter_type, cutter_blade_length, cutter_radius, cutter_battery_run_time,
                    cutter_battery_charge_time, cutter_velocity, simulation_verbosity,
                    simulation_show_progress, simulation_track_center, simulation_step_size,
                    simulation_perturb_at_bounces, simulation_perturb_segment,
                    simulation_perturb_segment_percent, simulation_parallel_processing,
                    start_position_x, start_position_y, start_direction_dirx,
                    start_direction_diry, start_direction_angle_deg, grid_hor_cells,
                    grid_ver_cells, grid_total_cells, grid_width_units, grid_height_units,
                    grid_cell_size, grid_map_file_name, grid_obstacles_num_obstacles,
                    grid_obstacles_cells_with_obstacle, grid_obstacles_percent,
                    image_image_size_width, image_image_size_height, image_image_file_name,
                    image_dpi, image_paper_size_name, image_paper_size_width_mm,
                    image_paper_size_height_mm, image_show_gridlines, stop_conditions_bounces,
                    stop_conditions_time_seconds, stop_conditions_coverage_percent,
                    stop_conditions_simulation_steps, stop_conditions_distance_units
                ) VALUES (
                    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                    ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38,
                    ?39, ?40, ?41, ?42
                )",
                params![
                    // Cutter
                    get_string_from_json(&model_data["Cutter"]["Type"]),
                    get_f64_from_json(&model_data["Cutter"]["Blade Length"]),
                    get_f64_from_json(&model_data["Cutter"]["Radius"]),
                    get_f64_from_json(&model_data["Cutter"]["Battery"]["Run Time"]),
                    get_f64_from_json(&model_data["Cutter"]["Battery"]["Charge Time"]),
                    get_f64_from_json(&model_data["Cutter"]["Velocity"]),
                    // Simulation
                    get_i64_from_json(&model_data["Simulation"]["Verbosity"]),
                    get_bool_as_i64_from_json(&model_data["Simulation"]["Show Progress"]),
                    get_bool_as_i64_from_json(&model_data["Simulation"]["Track Center"]),
                    get_f64_from_json(&model_data["Simulation"]["Step Size"]),
                    get_bool_as_i64_from_json(&model_data["Simulation"]["Perturb at Bounces"]),
                    get_bool_as_i64_from_json(&model_data["Simulation"]["Perturb Segment"]),
                    get_f64_from_json(&model_data["Simulation"]["Perturb Segment Percent"]),
                    get_bool_as_i64_from_json(&model_data["Simulation"]["Parallel Processing"]),
                    // Start
                    get_f64_from_json(&model_data["Start"]["Position"]["X"]),
                    get_f64_from_json(&model_data["Start"]["Position"]["Y"]),
                    get_f64_from_json(&model_data["Start"]["Direction"]["DirX"]),
                    get_f64_from_json(&model_data["Start"]["Direction"]["DirY"]),
                    get_f64_from_json(&model_data["Start"]["Direction"]["Angle (deg)"]),
                    // Grid
                    get_i64_from_json(&model_data["Grid"]["Hor. Cells"]),
                    get_i64_from_json(&model_data["Grid"]["Ver. Cells"]),
                    get_i64_from_json(&model_data["Grid"]["Total Cells"]),
                    get_f64_from_json(&model_data["Grid"]["Width (units)"]),
                    get_f64_from_json(&model_data["Grid"]["Height (units)"]),
                    get_f64_from_json(&model_data["Grid"]["Cell Size"]),
                    get_string_from_json(&model_data["Grid"]["Map File Name"]),
                    get_i64_from_json(&model_data["Grid"]["Obstacles"]["Num obstacles"]),
                    get_i64_from_json(&model_data["Grid"]["Obstacles"]["Cells with obstacle"]),
                    get_f64_from_json(&model_data["Grid"]["Obstacles"]["Percent"]),
                    // Image
                    get_i64_from_json(&model_data["Image"]["Image Size (mm)"]["Width"]),
                    get_i64_from_json(&model_data["Image"]["Image Size (mm)"]["Height"]),
                    get_string_from_json(&model_data["Image"]["Image File Name"]),
                    get_i64_from_json(&model_data["Image"]["DPI"]),
                    get_string_from_json(&model_data["Image"]["Paper Size"]["name"]),
                    get_f64_from_json(&model_data["Image"]["Paper Size"]["width_mm"]),
                    get_f64_from_json(&model_data["Image"]["Paper Size"]["height_mm"]),
                    get_bool_as_i64_from_json(&model_data["Image"]["Show Gridlines"]),
                    // Stop Conditions
                    get_i64_from_json(&model_data["Stop Conditions"]["Bounces"]),
                    get_f64_from_json(&model_data["Stop Conditions"]["Time (seconds)"]),
                    get_f64_from_json(&model_data["Stop Conditions"]["Coverage (%)"]),
                    get_i64_from_json(&model_data["Stop Conditions"]["Simulation Steps"]),
                    get_f64_from_json(&model_data["Stop Conditions"]["Distance (units)"]),
                ],
            )?;
            tx.last_insert_rowid()
        };

        // Store result with foreign key reference
        let result_id = {
            let result_json = model.get_simulation_result_as_json();
            let result_data = &result_json["Simulation Result"];

            tx.execute(
                "INSERT INTO results (
                    model_id, coverage_percent, coverage_cells, coverage_bounces,
                    coverage_max_visited, coverage_min_visited, cutter_type,
                    cutter_blade_length, cutter_radius, cutter_velocity, cutter_distance,
                    cutter_cells_covered, cutter_battery_run_time, cutter_battery_charge_time,
                    cutter_battery_charge_count, cutter_battery_charge_left_percent,
                    time_cpu_time, time_cutting_time, time_min_cov_time, time_efficiency,
                    start_position_x, start_position_y, start_direction_x, start_direction_y,
                    start_angle_degrees, grid_hor_cells, grid_vert_cells, grid_total_cells,
                    grid_obstacles_num_cells, grid_obstacles_percent, grid_cell_side_units,
                    grid_width_units, grid_height_units, steps_total, steps_size_units,
                    steps_per_cell, output_image_paper_size_name, output_image_paper_size_width_mm,
                    output_image_paper_size_height_mm, output_image_show_gridlines,
                    output_image_file_name, output_image_dpi, output_image_pixels_width,
                    output_image_pixels_height
                ) VALUES (
                    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20,
                    ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34, ?35, ?36, ?37, ?38,
                    ?39, ?40, ?41, ?42, ?43, ?44
                )",
                params![
                    model_id,
                    // Coverage
                    get_f64_from_json(&result_data["Coverage"]["Percent"]),
                    get_i64_from_json(&result_data["Coverage"]["Cells"]),
                    get_i64_from_json(&result_data["Coverage"]["Bounces"]),
                    get_i64_from_json(&result_data["Coverage"]["Max visited"]),
                    get_i64_from_json(&result_data["Coverage"]["Min visited"]),
                    // Cutter
                    get_string_from_json(&result_data["Cutter"]["Type"]),
                    get_f64_from_json(&result_data["Cutter"]["Blade Length"]),
                    get_f64_from_json(&result_data["Cutter"]["Radius"]),
                    get_f64_from_json(&result_data["Cutter"]["Velocity"]),
                    get_f64_from_json(&result_data["Cutter"]["Distance"]),
                    get_f64_from_json(&result_data["Cutter"]["Cells covered"]),
                    get_f64_from_json(&result_data["Cutter"]["Battery"]["Run time"]),
                    get_f64_from_json(&result_data["Cutter"]["Battery"]["Charge time"]),
                    get_i64_from_json(&result_data["Cutter"]["Battery"]["Charge count"]),
                    get_f64_from_json(&result_data["Cutter"]["Battery"]["Charge left (%)"]),
                    // Time
                    get_string_from_json(&result_data["Time"]["CPU time"]),
                    get_string_from_json(&result_data["Time"]["Cutting time"]),
                    get_string_from_json(&result_data["Time"]["Min.Cov.Time"]),
                    get_f64_from_json(&result_data["Time"]["Efficiency"]),
                    // Start
                    get_f64_from_json(&result_data["Start"]["Position"]["X"]),
                    get_f64_from_json(&result_data["Start"]["Position"]["Y"]),
                    get_f64_from_json(&result_data["Start"]["Direction"]["X"]),
                    get_f64_from_json(&result_data["Start"]["Direction"]["Y"]),
                    get_f64_from_json(&result_data["Start"]["Angle (degrees)"]),
                    // Grid
                    get_i64_from_json(&result_data["Grid"]["Hor.Cells"]),
                    get_i64_from_json(&result_data["Grid"]["Vert.Cells"]),
                    get_i64_from_json(&result_data["Grid"]["Total cells"]),
                    get_i64_from_json(&result_data["Grid"]["Obstacles"]["NumCells"]),
                    get_f64_from_json(&result_data["Grid"]["Obstacles"]["Percent"]),
                    get_f64_from_json(&result_data["Grid"]["Cell side (units)"]),
                    get_f64_from_json(&result_data["Grid"]["Width (units)"]),
                    get_f64_from_json(&result_data["Grid"]["Height (units)"]),
                    // Steps
                    get_i64_from_json(&result_data["Steps"]["Total"]),
                    get_f64_from_json(&result_data["Steps"]["Size (units)"]),
                    get_f64_from_json(&result_data["Steps"]["Per cell"]),
                    // Output image
                    get_string_from_json(&result_data["Output image"]["Paper size"]["name"]),
                    get_f64_from_json(&result_data["Output image"]["Paper size"]["width_mm"]),
                    get_f64_from_json(&result_data["Output image"]["Paper size"]["height_mm"]),
                    get_bool_as_i64_from_json(&result_data["Output image"]["Show gridlines"]),
                    get_string_from_json(&result_data["Output image"]["File name"]),
                    get_i64_from_json(&result_data["Output image"]["DPI"]),
                    get_i64_from_json(&result_data["Output image"]["Pixels"]["width"]),
                    get_i64_from_json(&result_data["Output image"]["Pixels"]["height"]),
                ],
            )?;
            tx.last_insert_rowid()
        };

        tx.commit()?;
        Ok((model_id, result_id))
    }
}

/// Utility function to store both model and results data from a simulation
pub fn store_simulation_to_database(model: &SimModel, db_path: &str) -> Result<(i64, i64), Box<dyn std::error::Error>> {
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
        JsonValue::Bool(b) => if *b { 1 } else { 0 },
        JsonValue::Number(n) => if n.as_i64().unwrap_or(0) != 0 { 1 } else { 0 },
        JsonValue::String(s) => if s.to_lowercase() == "true" { 1 } else { 0 },
        _ => 0,
    }
}
