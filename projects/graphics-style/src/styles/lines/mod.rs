use super::*;
mod width;

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
