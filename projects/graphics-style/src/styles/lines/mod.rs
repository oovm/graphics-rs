use super::*;
use crate::{GraphicsStyle, StyleContext};
mod width;

impl Default for LineWidth {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

pub struct EdgeWidth {
    pub value: f32,
}

impl GraphicsStyle for EdgeWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.value);
        state.polygon_edge_width = Some(self.value);
    }
}
