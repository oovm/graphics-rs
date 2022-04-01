use crate::{Drawable, GraphicsBackend};
use graphics_style::StyleContext;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct Graphics<'a> {
    pub graphic: Vec<Drawable<'a>>,
    pub setting: GraphicsSetting,
    pub style: StyleContext,
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
        Self { graphic: Vec::new(), setting: config.unwrap_or_default(), style: theme.unwrap_or_default() }
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
