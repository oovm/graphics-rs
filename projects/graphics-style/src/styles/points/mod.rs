use super::*;

mod add_assign;
mod size;

/// A point style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointStyle {
    /// The size of the point. [`PointSize`]
    pub point_size: Option<f32>,
    /// The color of the point. [`PointColor`]
    pub point_color: Option<RGBA>,
}

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
