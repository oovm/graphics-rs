use super::*;
mod width;

/// Line width
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineWidth {
    /// The line width
    pub value: f32,
}

/// Line color
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineColor {
    /// The line color
    pub value: Srgba,
}

impl Default for LineColor {
    fn default() -> Self {
        Self { value: Srgba::new(1.0, 1.0, 1.0, 1.0) }
    }
}
