use super::*;

mod methods;
mod style;
mod traits;

impl Point {
    /// Construct new point
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, ..Default::default() }
    }
}

impl Point {
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    /// Distance between two points.
    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    pub fn is_empty(&self, ctx: &StyleResolver) -> bool {
        let size = || self.get_size(ctx) <= 0.0;
        let color = || self.get_color(ctx).is_empty();
        size() || color()
    }
}
