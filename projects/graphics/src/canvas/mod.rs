use crate::Drawable;
use graphics_style::StyleResolver;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct Graphics {
    pub graphic: Vec<Box<dyn Drawable>>,
    pub style: StyleResolver,
}

impl Debug for Box<dyn Drawable> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
