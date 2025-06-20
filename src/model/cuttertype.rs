use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
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
