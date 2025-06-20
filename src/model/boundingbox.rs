use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

impl BoundingBox {
    pub(crate) fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
    pub fn init(width: f64, height: f64, radius: f64) -> Self {
        let min_x = radius;
        let max_x = width - radius;
        let min_y = radius;
        let max_y = height - radius;
        Self::new(min_x, max_x, min_y, max_y)
    }
    pub fn limit_x(&self, x: f64) -> f64 {
        x.max(self.min_x).min(self.max_x)
    }
    pub fn limit_y(&self, y: f64) -> f64 {
        y.max(self.min_y).min(self.max_y)
    }

    #[allow(dead_code)]
    pub fn limit(&self, x: f64, y: f64) -> (f64, f64) {
        (self.limit_x(x), self.limit_y(y))
    }
}
