// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

mod canvas;
mod elements;
mod errors;
mod traits;

pub use crate::{canvas::*, elements::*, errors::*, traits::*};
use graphics_style::{GraphicsStyle, StyleResolver, StyleSetter};

#[derive(Debug)]
pub enum Shape {
    // Circle(Circle),
    // Rectangle(Rectangle),
    Point(Point),
    Line(Line),
    // Polygon(Polygon),
    // Text(Text),
}

#[derive(Debug)]
pub enum Drawable {
    Shape(Shape),
    Style(GraphicsStyle),
}

impl Clone for Drawable {
    fn clone(&self) -> Self {
        todo!()
    }
}

pub trait GraphicsBackend {
    fn draw(&mut self, style: &mut StyleResolver, drawable: &Drawable) -> Result<(), GraphicsError> {
        match drawable {
            Drawable::Style(inner) => Ok(inner.set_local_style(style)),
            Drawable::Shape(inner) => match inner {
                Shape::Point(s) => self.draw_point(style, s),
                Shape::Line(s) => self.draw_line(style, s),
            },
        }
    }
    fn draw_point(&mut self, style: &StyleResolver, shape: &Point) -> Result<(), GraphicsError>;
    fn draw_line(&mut self, style: &StyleResolver, shape: &Line) -> Result<(), GraphicsError>;
}
