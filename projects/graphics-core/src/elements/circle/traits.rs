use super::*;
use crate::GraphicsShape;

impl From<(f32, f32)> for Point {
    fn from(point: (f32, f32)) -> Self {
        Self { x: point.0 as f32, y: point.1 as f32, ..Default::default() }
    }
}

impl Default for Circle {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, radius: 1.0, color: None }
    }
}

impl From<Point> for Circle {
    fn from(v: Point) -> Self {
        Self { x: v.x, y: v.y, radius: v.size.unwrap_or(1.0), color: v.color }
    }
}

impl From<Point> for GraphicsShape {
    fn from(v: Point) -> Self {
        Self::Circle(v.into())
    }
}

impl From<Circle> for GraphicsShape {
    fn from(v: Circle) -> Self {
        Self::Circle(v)
    }
}
