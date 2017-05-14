// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

mod canvas;
mod elements;
mod errors;
mod traits;

pub use crate::{canvas::*, elements::*, errors::*, traits::*};
use graphics_style::GraphicsStyle;

pub enum Shape {
    // Circle(Circle),
    // Rectangle(Rectangle),
    Line(Line),
    // Polygon(Polygon),
    // Text(Text),
}

pub trait Drawable {}

impl Clone for Shape {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Drawable for Shape {}

impl Drawable for dyn GraphicsStyle {}

pub trait GraphicsBackend {
    fn draw(&mut self, ctx: &mut GraphicsContext, drawable: dyn Drawable) -> Result<(), GraphicsError>;
}
