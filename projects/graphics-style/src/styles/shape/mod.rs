use std::ops::{Deref, DerefMut};

///
#[derive(Clone, Debug, PartialEq)]
pub struct StyledShape<Shape, Style> {
    shape: Shape,
    style: Style,
}

impl<Shape, Style> Deref for StyledShape<Shape, Style> {
    type Target = Shape;

    fn deref(&self) -> &Self::Target {
        &self.shape
    }
}

impl<Shape, Style> DerefMut for StyledShape<Shape, Style> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.shape
    }
}
