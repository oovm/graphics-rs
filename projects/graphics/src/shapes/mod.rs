// mod circle;
mod line;
mod point;

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
    size: Option<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointSize {
    pub value: f32,
}

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line {
    start: Point,
    end: Point,
}
