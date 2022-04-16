use super::*;

impl Square {
    pub fn new(x1: f32, y1: f32, side: f32) -> Self {
        Self { shape: graphics_shape::Square { anchor: Point { x: x1, y: y1 }, side } }
    }
}
