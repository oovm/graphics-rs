use super::*;

impl From<Triangle> for Polygon {
    fn from(v: Triangle) -> Self {
        Self { vertex: vec![], color: None }
    }
}

impl From<Square> for Polygon {
    fn from(v: Square) -> Self {
        Self { vertex: vec![], color: None }
    }
}

impl From<Rectangle> for Polygon {
    fn from(v: Rectangle) -> Self {
        Self { vertex: vec![], color: None }
    }
}

impl AsRef<Polygon> for Rectangle {
    fn as_ref(&self) -> &Polygon {
        &Polygon { vertex: vec![], color: None }
    }
}
