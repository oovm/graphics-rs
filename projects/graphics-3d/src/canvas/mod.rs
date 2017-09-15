use crate::{Drawable, GraphicsBackend};
use graphics_style::{StyleContext, StyleResolver};
use std::fmt::Debug;

#[derive(Debug, Default)]
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

impl Graphics {
    pub fn push<T>(&mut self, drawable: T)
    where
        T: Into<Drawable>,
    {
        self.graphic.push(drawable.into());
    }
}

impl Graphics {
    pub fn new(theme: Option<StyleContext>, config: Option<GraphicsSetting>) -> Self {
        Self {
            graphic: Vec::new(),
            setting: config.unwrap_or_default(),
            style: StyleResolver::with_theme_style(theme.unwrap_or_default()),
        }
    }
    pub fn render_with<T>(&self, backend: &mut T) -> Result<<T as GraphicsBackend>::Output, <T as GraphicsBackend>::Error>
    where
        T: GraphicsBackend,
    {
        let mut state = self.style.clone();
        backend.on_start(self, &mut state)?;
        for drawable in &self.graphic {
            backend.draw(self, &mut state, drawable)?;
        }
        backend.on_finish(self, &mut state)?;
        backend.get_output(self)
    }
    pub fn clear(&mut self) {
        self.graphic.clear();
    }
}

impl Default for GraphicsSetting {
    fn default() -> Self {
        Self { width: 100.0, height: 100.0 }
    }
}
