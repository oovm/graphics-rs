use super::*;
mod methods;
mod style;
mod traits;
use crate::GraphicsContext;

impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, size: None }
    }
}

impl Point {
    /// Construct new point
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, ..Default::default() }
    }
    /// Set point size

    /// Distance between two points.
    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn is_empty(&self, ctx: &GraphicsContext) -> bool {
        let size = self.size.unwrap_or(ctx.point_size) <= 0.0;
        size
    }
}

impl From<(f32, f32)> for Point {
    fn from(point: (f32, f32)) -> Self {
        Self { x: point.0 as f32, y: point.1 as f32, ..Default::default() }
    }
}
