use super::*;
use crate::GraphicsShape;

impl Default for Square {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, side: 1.0, color: None }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, width: 1.618, height: 1.0, color: None }
    }
}

impl From<Square> for Rectangle {
    fn from(v: Square) -> Self {
        Self { x: v.x, y: v.y, width: v.side, height: v.side, color: v.color }
    }
}

impl From<Rectangle> for GraphicsShape {
    fn from(v: Rectangle) -> Self {
        Self::Rectangle(v)
    }
}

impl From<Square> for GraphicsShape {
    fn from(v: Square) -> Self {
        Rectangle::from(v).into()
    }
}
