mod setting;
pub use self::setting::GraphicsSetting;
use crate::{GraphicElement, GraphicsBackend};
use graphics_shape::Ellipse;
use graphics_style::{EdgeStyle, EllipseStyle, FillStyle, StyleContext};
use std::fmt::Debug;

#[derive(Debug, Default)]
pub struct Graphics {
    pub graphic: Vec<GraphicElement>,
    pub style: StyleContext,
    pub setting: GraphicsSetting,
}

impl Clone for GraphicElement {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Graphics {
    pub fn push<T>(&mut self, drawable: T)
    where
        T: Into<GraphicElement>,
    {
        self.graphic.push(drawable.into());
    }
}

impl Graphics {
    pub fn new(theme: Option<StyleContext>, config: Option<GraphicsSetting>) -> Self {
        Self { graphic: Vec::new(), setting: config.unwrap_or_default(), style: theme.unwrap_or_default() }
    }

    pub fn render_with<T>(&mut self, backend: &mut T) -> Result<<T as GraphicsBackend>::Output, <T as GraphicsBackend>::Error>
    where
        T: GraphicsBackend,
    {
        backend.on_start(self)?;
        let setting = &self.setting;
        let style = &mut self.style;
        for drawable in &mut self.graphic {
            render_dispatch(setting, style, drawable, backend)?;
        }
        backend.on_finish(self)?;
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

fn render_dispatch<T>(
    setting: &GraphicsSetting,
    context: &mut StyleContext,
    drawable: &mut GraphicElement,
    backend: &mut T,
) -> Result<(), <T as GraphicsBackend>::Error>
where
    T: GraphicsBackend,
{
    match drawable {
        GraphicElement::StyleChange { style, finish } => {
            style.change_style(context);
            *finish = true;
            Ok(())
        }
        GraphicElement::Point(point) => {
            todo!()
            // let shape = Ellipse::new(*shape, (style.size, style.size), 0.0);
            // let style = EllipseStyle { fill: FillStyle::from(style.color), edge: EdgeStyle::empty() };
            // backend.draw_ellipse(setting, &shape, &style)
        }
        GraphicElement::Circle { shape, style, state: _ } => {
            let shape = Ellipse::new(shape.center, (shape.radius, shape.radius), 0.0);
            let style = EllipseStyle::from(style.clone());
            backend.draw_ellipse(setting, &shape, &style)
        }
        GraphicElement::Ellipse { shape, style, state: _ } => backend.draw_ellipse(setting, shape, style),
        GraphicElement::Sequences(v) => {
            for drawable in v {
                render_dispatch(setting, context, drawable, backend)?;
            }
            Ok(())
        }
    }
}
