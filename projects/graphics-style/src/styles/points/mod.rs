use crate::{GraphicsStyle, StyleResolver};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointSize {
    pub value: f32,
}

impl Default for PointSize {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}
