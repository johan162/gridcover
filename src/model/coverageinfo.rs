// A struct to track information about cell coverage
#[derive(Debug, Clone, Copy)]
pub struct CoverageInfo {
    pub segment_number: usize, // Which bounce iteration covered this cell
    pub times_visited: usize, // How many times this cell was covered
}

impl CoverageInfo {
    pub fn new(segment_number: usize, times_visited: usize) -> Self {
        Self {
            segment_number,
            times_visited,
        }
    }
}
