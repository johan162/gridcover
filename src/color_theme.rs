use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ColorTheme {
    pub name: String,
    pub grid_background_color: [u8; 3],
    pub grid_line_color: [u8; 3],
    pub text_color: [u8; 3],
    pub text_background_adjustment: f32,
    pub obstacle_color: [u8; 3],
    pub center_color: [u8; 3],
    pub coverage_shades: Vec<[u8; 3]>,
}

impl ColorTheme {
    /// Get a coverage color based on the number of times visited
    pub fn get_coverage_color(&self, times_visited: usize) -> [u8; 3] {
        let color_idx = times_visited.min(self.coverage_shades.len() - 1);
        self.coverage_shades[color_idx]
    }

    /// Get the maximum number of coverage levels
    #[allow(dead_code)]
    pub fn max_coverage_levels(&self) -> usize {
        self.coverage_shades.len()
    }
}

pub struct ColorThemeManager {
    themes: HashMap<String, ColorTheme>,
    default_theme: String,
}

impl ColorThemeManager {
    pub fn new() -> Self {
        let mut manager = ColorThemeManager {
            themes: HashMap::new(),
            default_theme: "default".to_string(),
        };

        // Register built-in themes
        manager.register_default_theme();
        manager.register_extended_green_theme();
        manager.register_blue_theme();
        manager.register_pure_green_theme();
        manager.register_gray_green_theme();
        manager.register_high_contrast_theme();

        manager
    }

    /// Register a new color theme
    pub fn register_theme(&mut self, theme: ColorTheme) {
        self.themes.insert(theme.name.clone(), theme);
    }

    /// Get a theme by name, falls back to default if not found
    pub fn get_theme(&self, name: &str) -> &ColorTheme {
        self.themes
            .get(name)
            .unwrap_or_else(|| self.themes.get(&self.default_theme).unwrap())
    }

    /// Get the default theme
    #[allow(dead_code)]
    pub fn get_default_theme(&self) -> &ColorTheme {
        self.themes.get(&self.default_theme).unwrap()
    }

    /// List all available theme names
    pub fn list_theme_names(&self) -> Vec<&String> {
        self.themes.keys().collect()
    }

    /// Check if a string is a valid color theme name
    pub fn is_valid_theme_name(&self, name: &str) -> bool {
        self.themes.contains_key(name)
    }

    /// Set the default theme
    #[allow(dead_code)]
    pub fn set_default_theme(&mut self, name: &str) -> Result<(), String> {
        if self.themes.contains_key(name) {
            self.default_theme = name.to_string();
            Ok(())
        } else {
            Err(format!("Theme '{name}' not found"))
        }
    }

    // Built-in theme definitions
    fn register_default_theme(&mut self) {
        let theme = ColorTheme {
            name: "default".to_string(),
            grid_background_color: [150, 150, 150],
            text_color: [255, 255, 255],
            text_background_adjustment: 0.4,
            grid_line_color: [0, 0, 0],
            obstacle_color: [150, 0, 0],
            center_color: [0, 0, 0],
            coverage_shades: vec![
                [240, 255, 240], // Honeydew (very light green)
                [220, 255, 220],
                [200, 255, 200],
                [180, 255, 180],
                [160, 255, 160],
                [140, 255, 140],
                [120, 255, 120],
                [100, 255, 100],
                [80, 220, 80],
                [60, 200, 60],
                [40, 180, 40],
                [30, 160, 30],
                [20, 140, 20],
                [15, 120, 15],
                [10, 100, 10],
                [8, 80, 8],
                [6, 60, 6],
                [4, 40, 4],
                [2, 20, 2],
                [0, 64, 0],
                [0, 44, 0], // Pure dark green
            ],
        };
        self.register_theme(theme);
    }

    fn register_extended_green_theme(&mut self) {
        let theme = ColorTheme {
            name: "green30".to_string(),
            grid_background_color: [150, 150, 150],
            text_color: [255, 255, 255],
            text_background_adjustment: 0.4,
            grid_line_color: [0, 0, 0],
            obstacle_color: [150, 0, 0],
            center_color: [0, 0, 0],
            coverage_shades: vec![
                [240, 255, 240], // Honeydew (very light green)
                [230, 250, 230],
                [220, 255, 220],
                [210, 245, 210],
                [200, 255, 200],
                [190, 245, 190],
                [180, 255, 180],
                [170, 245, 170],
                [160, 255, 160],
                [150, 245, 150],
                [140, 255, 140],
                [130, 245, 130],
                [120, 255, 120],
                [110, 245, 110],
                [100, 255, 100],
                [90, 230, 90],
                [80, 220, 80],
                [70, 210, 70],
                [60, 200, 60],
                [50, 190, 50],
                [40, 180, 40],
                [35, 170, 35],
                [30, 160, 30],
                [25, 150, 25],
                [20, 140, 20],
                [15, 120, 15],
                [10, 100, 10],
                [8, 80, 8],
                [4, 60, 4],
                [0, 44, 0], // Very dark green
            ],
        };
        self.register_theme(theme);
    }

    fn register_pure_green_theme(&mut self) {
        let theme = ColorTheme {
            name: "pure_green".to_string(),
            grid_background_color: [150, 150, 150],
            text_color: [255, 255, 255],
            text_background_adjustment: 0.4,
            grid_line_color: [0, 0, 0],
            obstacle_color: [150, 0, 0],
            center_color: [0, 0, 0],
            coverage_shades: vec![
                [240, 255, 240],
                [230, 250, 230],
                [220, 245, 220],
                [210, 240, 210],
                [200, 235, 200],
                [190, 230, 190],
                [180, 225, 180],
                [170, 220, 170],
                [160, 215, 160],
                [150, 210, 150],
                [140, 205, 140],
                [130, 200, 130],
                [120, 195, 120],
                [110, 190, 110],
                [100, 185, 100],
                [90, 180, 90],
                [80, 175, 80],
                [70, 170, 70],
                [60, 165, 60],
                [50, 160, 50],
                [40, 155, 40],
                [30, 150, 30],
                [25, 135, 25],
                [20, 125, 20],
                [18, 115, 18],
                [16, 105, 16],
                [14, 95, 14],
                [12, 85, 12],
                [10, 75, 10],
                [8, 65, 8],
            ],
        };
        self.register_theme(theme);
    }

    fn register_gray_green_theme(&mut self) {
        let theme = ColorTheme {
            name: "gray_green".to_string(),
            grid_background_color: [150, 150, 150],
            text_color: [255, 255, 255],
            text_background_adjustment: 0.2,
            grid_line_color: [0, 0, 0],
            obstacle_color: [150, 0, 0],
            center_color: [0, 0, 0],
            coverage_shades: vec![
                [240, 255, 240],
                [230, 250, 230],
                [220, 245, 220],
                [210, 240, 210],
                [200, 235, 200],
                [190, 230, 190],
                [180, 225, 180],
                [170, 220, 170],
                [160, 215, 160],
                [150, 210, 150],
                [140, 205, 140],
                [130, 200, 130],
                [120, 195, 120],
                [110, 190, 110],
                [100, 185, 100],
                [90, 180, 90],
                [80, 175, 80],
                [70, 170, 70],
                [60, 165, 60],
                [50, 160, 50],
                [40, 155, 40],
                [30, 150, 30],
                [25, 135, 25],
                [20, 125, 20],
                [18, 115, 18],
                [16, 105, 16],
                [14, 95, 14],
                [12, 85, 12],
                [10, 75, 10],
                [8, 65, 8],
            ],
        };
        self.register_theme(theme);
    }

    fn register_blue_theme(&mut self) {
        let theme = ColorTheme {
            name: "blue".to_string(),
            grid_background_color: [150, 150, 150],
            text_color: [255, 255, 255],
            text_background_adjustment: 0.3,
            grid_line_color: [0, 0, 0],
            obstacle_color: [150, 0, 0],
            center_color: [0, 0, 0],
            coverage_shades: vec![
                [240, 248, 255], // Alice blue (very light)
                [220, 235, 255],
                [200, 220, 255],
                [180, 205, 255],
                [160, 190, 255],
                [140, 175, 255],
                [120, 160, 255],
                [100, 145, 255],
                [80, 130, 220],
                [60, 115, 200],
                [40, 100, 180],
                [30, 85, 160],
                [20, 70, 140],
                [15, 55, 120],
                [10, 40, 100],
                [8, 30, 80],
                [6, 20, 60],
                [4, 15, 40],
                [2, 10, 20],
                [0, 0, 64],
                [0, 0, 44], // Pure dark blue
            ],
        };
        self.register_theme(theme);
    }

    fn register_high_contrast_theme(&mut self) {
        let theme = ColorTheme {
            name: "orange_red".to_string(),
            grid_background_color: [255, 255, 255],
            text_color: [255, 255, 255],
            text_background_adjustment: 0.1,
            grid_line_color: [128, 128, 128],
            obstacle_color: [0, 0, 255],
            center_color: [0, 0, 0],
            coverage_shades: vec![
                [255, 255, 0], // Yellow
                [255, 200, 0], // Orange-yellow
                [255, 150, 0], // Orange
                [255, 100, 0], // Red-orange
                [255, 50, 0],  // Red
                [200, 0, 0],   // Dark red
                [150, 0, 0],   // Darker red
                [100, 0, 0],   // Very dark red
                [50, 0, 0],    // Almost black red
                [0, 0, 0],     // Black
            ],
        };
        self.register_theme(theme);
    }
}

impl Default for ColorThemeManager {
    fn default() -> Self {
        Self::new()
    }
}
