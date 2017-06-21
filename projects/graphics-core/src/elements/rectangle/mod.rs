use super::*;
mod style;
mod traits;

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height, ..Default::default() }
    }
}

impl Rectangle {
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
    pub fn get_width(&self) -> f32 {
        self.width
    }
    pub fn get_height(&self) -> f32 {
        self.height
    }
    pub fn is_empty(&self, ctx: &StyleResolver) -> bool {
        let size = || self.width <= 0.0 || self.height <= 0.0;
        let color = || self.get_color(ctx).is_empty();
        size() || color()
    }
}
