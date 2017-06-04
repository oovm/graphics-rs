// mod circle;
mod line;
mod point;

use graphics_style::{LineColor, LineWidth, PointSize, StyleResolver, RGBA};

pub struct Pixel {
    pub x: u32,
    pub y: u32,
    pub color: RGBA,
}

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point {
    x: f32,
    y: f32,
    size: Option<PointSize>,
    color: Option<RGBA>,
}

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Line {
    start: Point,
    end: Point,
    width: Option<LineWidth>,
    color: Option<LineColor>,
}
