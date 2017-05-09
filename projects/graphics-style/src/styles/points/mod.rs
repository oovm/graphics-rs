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

impl GraphicsStyle for PointSize {
    fn set_style(&self, context: &mut StyleResolver) {
        context.local.point_size = self.0;
    }
}
