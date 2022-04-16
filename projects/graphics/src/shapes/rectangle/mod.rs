use super::*;
mod rectangles;
mod square;
mod style;
mod traits;

pub struct Square {
    shape: graphics_shape::Square,
}

pub struct Rectangle {
    shape: graphics_shape::Rectangle,
    style: graphics_style::RectangleStyle,
}
