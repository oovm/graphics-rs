use crate::Drawable;
use graphics_style::StyleResolver;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct Graphics {
    pub graphic: Vec<Drawable>,
    pub style: StyleResolver,
}
