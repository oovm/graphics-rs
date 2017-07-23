mod point;

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
    Point(Point),
    Line(Line),
    // Circle(Circle),
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
