use super::*;
mod width;

/// A point style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineStyle {
    /// The size of the point. [`PointSize`]
    pub line_width: Option<f32>,
    /// The color of the point. [`PointColor`]
    pub line_color: Option<RGBA>,
}

/// Line width
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineWidth {
    /// The line width
    pub value: f32,
}

/// Line width
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LineColor {
    /// The line width
    pub value: RGBA,
}
