use crate::{Drawable, PointSize};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Graphics {
    pub graphic: Vec<Drawable>,
    pub context: GraphicsContext,
    pub theme: GraphicsContext,
}

#[derive(Debug, Clone)]
pub struct GraphicsContext {
    pub point_size: Option<PointSize>,
}

impl Graphics {
    pub fn point_size(&self) -> PointSize {
        self.theme.point_size.unwrap_or(self.context.point_size.unwrap_or(PointSize::default()))
    }
}
