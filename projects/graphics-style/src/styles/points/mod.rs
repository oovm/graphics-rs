use super::*;
mod size;

/// A point style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointSize {
    /// The size of the point.
    pub value: f32,
}

/// Line width
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PointColor {
    /// The line width
    pub value: RGBA,
}
