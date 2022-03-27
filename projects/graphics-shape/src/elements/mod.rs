mod point;
// mod circle;
mod rectangle;

pub use point::{Point, Point3D};

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

/// A rectangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    pub x: f32,
    pub y: f32,
    pub side: f32,
}

/// A rectangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
