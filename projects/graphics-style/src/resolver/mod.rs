mod content;

pub use self::content::StyleContext;
use crate::*;

/// Resolve missing style
#[derive(Debug, Clone, Default)]
pub struct StyleResolver {
    theme: StyleContext,
    local: StyleContext,
}

impl StyleResolver {
    /// Set the style of the given element.
    pub fn set_theme_style(&mut self, theme: StyleContext) {
        self.theme = theme;
    }
    /// Set the style of the given element.
    pub fn set_local_style<T>(&mut self, style: T)
    where
        T: GraphicsStyle,
    {
        style.draw_style(&mut self.local);
    }
}
