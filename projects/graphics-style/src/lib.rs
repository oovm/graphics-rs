use self::context::StyleContext;
pub use self::styles::*;

mod context;
mod styles;

pub trait GraphicsStyle {
    fn set_local_style(&self, context: &mut StyleResolver);
}

#[derive(Debug, Clone)]
pub struct StyleResolver {
    theme: StyleContext,
    local: StyleContext,
}
