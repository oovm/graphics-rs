/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point {
    x: f32,
    y: f32,
    size: Option<f32>,
    color: Option<RGBA>,
}

/// A 3D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point3D {
    x: f32,
    y: f32,
    z: f32,
    size: Option<f32>,
    color: Option<RGBA>,
}

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: Option<RGBA>,
}

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ball {
    x: f32,
    y: f32,
    z: f32,
    radius: f32,
    color: Option<RGBA>,
}

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Disk {
    x: f32,
    y: f32,
    radius: f32,
    color: Option<RGBA>,
}

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    width: Option<f32>,
    color: Option<RGBA>,
}

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Triangle {
    vertex: [(f32, f32); 3],
    color: Option<RGBA>,
}

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Parallelogram {
    vertex: [(f32, f32); 4],
    color: Option<RGBA>,
}

/// A circle.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Polygon {
    vertex: Vec<(f32, f32)>,
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
    Polygon(Polygon),
    // Text(Text),
}

impl GraphicsShape {
    pub fn is_empty(&self, state: &StyleResolver) -> bool {
        true
        // match self {
        //     Self::Pixel(s) => s.is_empty(state),
        //     Self::Point(s) => s.is_empty(state),
        //     Self::Line(s) => s.is_empty(state),
        //     Self::Rectangle(s) => s.is_empty(state),
        // }
    }
}
