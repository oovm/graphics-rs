use super::*;

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
    pub value: Srgb,
}

impl Default for LineWidth {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Default for LineColor {
    fn default() -> Self {
        Self { value: Srgb::new(1.0, 1.0, 1.0) }
    }
}
