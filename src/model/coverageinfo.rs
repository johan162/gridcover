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
