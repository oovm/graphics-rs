use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineWidth {
    pub value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineColor {
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
