use crate::{
    context::{StyleContext, StyleResolver},
    styles::PointSize,
};

mod context;
mod styles;

pub trait GraphicsStyle {
    fn set_style(&self, context: &mut StyleResolver);
}

#[derive(Debug, Clone)]
pub struct StyleResolver {
    theme: StyleContext,
    local: StyleContext,
}
