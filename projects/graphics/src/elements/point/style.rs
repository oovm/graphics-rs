use super::*;
use crate::GraphicsStyle;

impl GraphicsStyle for PointSize {
    fn draw(&self, context: &mut GraphicsContext) {
        todo!()
    }
}

impl Point {
    pub fn with_size<T>(self, size: T) -> Self
    where
        T: Into<PointSize>,
    {
        Self { size: Some(size.min(0.0)), ..self }
    }
}
