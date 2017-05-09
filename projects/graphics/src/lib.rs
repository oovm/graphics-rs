// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

mod canvas;
mod elements;
mod errors;
mod styles;
mod traits;

pub use crate::{canvas::*, elements::*, errors::*, styles::*, traits::*};

pub enum Shape {
    // Circle(Circle),
    // Rectangle(Rectangle),
    Line(Line),
    // Polygon(Polygon),
    // Text(Text),
}

pub enum Drawable {
    Shape(Shape),
    Style(Style),
}

pub trait GraphicsBackend {
    fn draw(&mut self, ctx: &mut GraphicsContext, drawable: Drawable) -> Result<(), GraphicsError>;
}
