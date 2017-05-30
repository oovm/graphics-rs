use super::*;
mod size;

/// A point style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointSize {
    /// The size of the point.
    pub value: f32,
}

/// A point style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointColor {
    /// The color of the point.
    pub value: Srgb,
}

impl Default for PointColor {
    fn default() -> Self {
        Self { value: Srgb::new(1.0, 1.0, 1.0) }
    }
}
