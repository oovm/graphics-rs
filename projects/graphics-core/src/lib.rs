// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

mod canvas;
mod elements;
mod errors;
mod traits;

pub use crate::{canvas::*, elements::*, errors::*, traits::*};
pub use graphics_style::{GraphicsStyle, StyleResolver};
pub use traits::GraphicsBackend;

#[derive(Debug)]
pub enum GraphicsShape {
    Pixel(Pixel),
    // Circle(Circle),
    // Rectangle(Rectangle),
    Point(Point),
    Line(Line),
    // Polygon(Polygon),
    // Text(Text),
}

#[derive(Debug)]
pub enum Drawable {
    Shape(GraphicsShape),
    Style(GraphicsStyle),
}

impl Clone for Drawable {
    fn clone(&self) -> Self {
        todo!()
    }
}
