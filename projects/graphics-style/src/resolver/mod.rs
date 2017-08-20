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
    pub fn get_theme_style(&self) -> &StyleContext {
        &self.theme
    }
    /// Set the style of the given element.
    pub fn set_theme_style(&mut self, theme: StyleContext) {
        self.theme = theme;
    }
    /// Set the style of the given element.
    pub fn with_theme_style(theme: StyleContext) -> Self {
        Self { theme, local: Default::default() }
    }

    pub fn get_local_style(&self) -> &StyleContext {
        &self.local
    }
    /// Set the style of the given element.
    pub fn set_local_style<T>(&mut self, style: T)
    where
        T: GraphicsStyle,
    {
        style.draw_style(&mut self.local);
    }
}
