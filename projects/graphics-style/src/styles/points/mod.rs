use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointSize {
    pub value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointColor {
    pub value: Srgb,
}

impl Default for PointSize {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Default for PointColor {
    fn default() -> Self {
        Self { value: Srgb::new(1.0, 1.0, 1.0) }
    }
}
