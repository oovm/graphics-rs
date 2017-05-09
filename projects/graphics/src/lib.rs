// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

mod canvas;
mod errors;
mod shapes;
mod styles;
mod traits;

pub use crate::{canvas::*, errors::*, shapes::*, styles::*, traits::*};

pub enum Shape {
    // Circle(Circle),
    // Rectangle(Rectangle),
    Line(Line),
    // Polygon(Polygon),
    // Text(Text),
}

pub enum Style {
    PointSize(PointSize),
}

pub enum Drawable {
    Shape(Shape),
    Style(Style),
}

impl GraphicsStyle for Style {
    fn set_style(&self, context: &mut GraphicsContext) {
        match self {
            Style::PointSize(v) => v.set_style(context),
        }
    }
}

impl GraphicsStyle for PointSize {
    fn set_style(&self, context: &mut GraphicsContext) {
        context.point_size = self.0;
    }
}

pub trait GraphicsStyle {
    fn set_style(&self, context: &mut GraphicsContext);
}

pub trait GraphicsBackend {
    fn draw(&mut self, ctx: &mut GraphicsContext, drawable: Drawable) -> Result<(), GraphicsError>;
}
