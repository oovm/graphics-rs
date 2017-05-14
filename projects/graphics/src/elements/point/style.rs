use super::*;
use crate::GraphicsStyle;

impl Point {
    pub fn with_size<T>(self, size: T) -> Self
    where
        T: Into<PointSize>,
    {
        Self { size: Some(size.into()), ..self }
    }
}
