// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

mod canvas;
mod elements;
mod errors;
mod traits;

pub use crate::{canvas::*, elements::*, errors::*, traits::*};
pub use graphics_style::{GraphicsStyle, StyleResolver};

#[derive(Debug)]
pub enum GraphicsShape {
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

#[allow(unused_variables)]
pub trait GraphicsBackend {
    type Output;

    fn get_output(&mut self, state: &Graphics) -> Result<Self::Output, GraphicsError>;

    fn on_start(&mut self, state: &mut Graphics) -> Result<(), GraphicsError> {
        Ok(())
    }
    fn on_finish(&mut self, state: &mut Graphics) -> Result<(), GraphicsError> {
        Ok(())
    }

    fn draw(&mut self, state: &mut Graphics) -> Result<(), GraphicsError> {
        for drawable in &state.graphic {
            match drawable {
                Drawable::Style(inner) => state.style.set_local_style(inner.clone()),
                Drawable::Shape(inner) => match inner {
                    GraphicsShape::Point(s) => self.draw_point(state, s)?,
                    GraphicsShape::Line(s) => self.draw_line(state, s)?,
                },
            }
        }
        Ok(())
    }
    fn draw_point(&mut self, context: &Graphics, shape: &Point) -> Result<(), GraphicsError>;
    fn draw_line(&mut self, context: &Graphics, shape: &Line) -> Result<(), GraphicsError>;
}
