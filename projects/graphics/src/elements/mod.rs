// mod circle;
mod color;
mod line;
mod point;

use graphics_style::{PointSize, StyleResolver};

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
    size: Option<PointSize>,
}

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    start: Point,
    end: Point,
}
