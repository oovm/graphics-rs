mod circle;
mod line;
mod pixel;
mod point;
mod rectangle;
mod square;
use graphics_style::{StyleResolver, RGBA};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
    pub x: f32,
    pub y: f32,
    pub c: RGBA,
}

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point {
    x: f32,
    y: f32,
    size: Option<f32>,
    color: Option<RGBA>,
}

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: Option<RGBA>,
}

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Line {
    start: Point,
    end: Point,
    width: Option<f32>,
    color: Option<RGBA>,
}

/// A rectangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Square {
    x: f32,
    y: f32,
    side: f32,
    color: Option<RGBA>,
}

/// A rectangle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Option<RGBA>,
}

#[derive(Debug)]
pub enum GraphicsShape {
    Pixel(Pixel),
    Line(Line),
    Circle(Circle),
    Rectangle(Rectangle),
    // Polygon(Polygon),
    // Text(Text),
}

impl GraphicsShape {
    pub fn is_empty(&self, state: &StyleResolver) -> bool {
        match self {
            Self::Pixel(s) => s.is_empty(state),
            Self::Point(s) => s.is_empty(state),
            Self::Line(s) => s.is_empty(state),
            Self::Rectangle(s) => s.is_empty(state),
        }
    }
}
