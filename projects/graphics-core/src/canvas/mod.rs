use crate::Drawable;
use graphics_style::StyleResolver;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Graphics {
    pub graphic: Vec<Drawable>,
    pub setting: GraphicsSetting,
    pub style: StyleResolver,
}

#[derive(Debug)]
pub struct GraphicsSetting {
    pub width: f32,
    pub height: f32,
}
