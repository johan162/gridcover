use serde::{Serialize, Deserialize};
use serde_json::json;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum PaperSize {
    A5,
    A4,
    A3,
    A2,
    A1,
    A0,
    Letter,
    Legal,
    Tabloid,
    Executive,
    Custom, // Custom size in mm
}

impl PaperSize {
    pub fn as_str(&self) -> &str {
        match self {
            PaperSize::A5 => "A5",
            PaperSize::A4 => "A4",
            PaperSize::A3 => "A3",
            PaperSize::A2 => "A2",
            PaperSize::A1 => "A1",
            PaperSize::A0 => "A0",
            PaperSize::Letter => "Letter",
            PaperSize::Legal => "Legal",
            PaperSize::Tabloid => "Tabloid",
            PaperSize::Executive => "Executive",
            PaperSize::Custom => "Custom",
        }
    }

    pub fn get_json(&self) -> serde_json::Value {
        json!({
            "format": self.as_str(),
            "width_mm": paper_size_to_mm(self.as_str()).map_or(0.0, |(w, _)| w),
            "height_mm": paper_size_to_mm(self.as_str()).map_or(0.0, |(_, h)| h),
        })
    }

    #[allow(dead_code)]
    pub fn get_size_pixels(&self, dpi: u32) -> Option<(u32, u32)> {
        paper_size_to_pixels(self.as_str(), dpi)
    }

    pub fn get_size_mm(&self) -> Option<(f64, f64)> {
        paper_size_to_mm(self.as_str())
    }
}

impl std::str::FromStr for PaperSize {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a5" => Ok(PaperSize::A5),
            "a4" => Ok(PaperSize::A4),
            "a3" => Ok(PaperSize::A3),
            "a2" => Ok(PaperSize::A2),
            "a1" => Ok(PaperSize::A1),
            "a0" => Ok(PaperSize::A0),
            "letter" => Ok(PaperSize::Letter),
            "legal" => Ok(PaperSize::Legal),
            "tabloid" => Ok(PaperSize::Tabloid),
            "executive" => Ok(PaperSize::Executive),
            "custom" => Ok(PaperSize::Custom),
            _ => Err(format!("Invalid paper size: {s}")),
        }
    }
}

pub fn paper_size_to_mm(paper_size: &str) -> Option<(f64, f64)> {
    match paper_size.to_lowercase().as_str() {
        "a5" => Some((148.0, 210.0)),
        "a4" => Some((210.0, 297.0)),
        "a3" => Some((297.0, 420.0)),
        "a2" => Some((420.0, 594.0)),
        "a1" => Some((594.0, 841.0)),
        "a0" => Some((841.0, 1189.0)),
        "letter" => Some((215.9, 279.4)),    // 8.5 x 11 inches
        "legal" => Some((215.9, 355.6)),     // 8.5 x 14 inches
        "tabloid" => Some((279.4, 431.8)),   // 11 x 17 inches
        "executive" => Some((191.0, 254.0)), // 7.25 x 10.
        "custom" => Some((0.0, 0.0)), // Custom size, will be set later
        _ => None,
    }
}

/// Convert paper size to pixels based on DPI (dots per inch)
/// Returns None if the paper size is invalid
pub fn paper_size_to_pixels(paper_size: &str, dpi: u32) -> Option<(u32, u32)> {
    paper_size_to_mm(paper_size).map(|(width_mm, height_mm)| {
        let width_px = (width_mm * dpi as f64 / 25.4) as u32;
        let height_px = (height_mm * dpi as f64 / 25.4) as u32;
        (width_px, height_px)
    })
}
