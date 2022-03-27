mod content;
pub use self::content::StyleContext;
use crate::*;
use serde::{Deserialize, Serialize};

/// Resolve when style is not specific or missing.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct StyleResolver {
    theme: StyleContext,
    local: StyleContext,
    once: StyleContext,
}

impl StyleResolver {
    /// Create a new style resolver
    pub fn get_theme_style(&self) -> &StyleContext {
        &self.theme
    }
    /// Set the style of the given element.
    pub fn set_theme_style(&mut self, theme: StyleContext) {
        self.theme = theme;
    }
    /// Set the style of the given element.
    pub fn with_theme_style(theme: StyleContext) -> Self {
        Self { theme, local: Default::default(), once: Default::default() }
    }
    /// Set the style of the given element.
    pub fn view_local_style(&self) -> &StyleContext {
        &self.local
    }
    /// Set the style of the given element.
    pub fn set_local_style<T>(&mut self, style: &[T])
    where
        T: GraphicsStyle,
    {
        for i in style {
            i.draw_style(&mut self.local);
        }
    }
    /// Set the style of the given element.
    pub fn set_once_style<T>(&mut self, style: &[T])
    where
        T: GraphicsStyle,
    {
        for i in style {
            i.draw_style(&mut self.local);
        }
    }
    /// Set the style of the given element.
    pub fn clean_once_style(&mut self) {
        self.once = Default::default();
    }
}
