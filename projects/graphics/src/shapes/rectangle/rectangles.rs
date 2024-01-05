use super::*;
use graphics_2d::{Drawable, GraphicElement};
use graphics_style::{FillStyle, RectangleStyle};

impl Rectangle {
    pub fn new(x1: f32, y1: f32, width: f32, height: f32) -> Self {
        Self {
            shape: graphics_shape::Rectangle { anchor: Point { x: x1, y: y1 }, side: (width, height) },
            style: RectangleStyle { fill: FillStyle { texture: Default::default() }, edge: Default::default() },
        }
    }
    pub fn get_width(&self) -> f32 {
        self.shape.side.0
    }
    pub fn get_height(&self) -> f32 {
        self.shape.side.1
    }
}

impl Drawable for Rectangle {
    fn draw(&self) -> GraphicElement {
        todo!()
    }
}
