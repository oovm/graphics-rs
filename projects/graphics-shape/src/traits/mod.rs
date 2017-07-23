mod convert;
#[cfg(feature = "wolfram_wxf")]
mod wolfram;
use crate::{Drawable, Graphics, GraphicsShape, Line, Pixel, Point, Rectangle};
use graphics_style::StyleResolver;

pub trait Distance {
    type Other;
    fn distance(&self, other: &Self::Other) -> f32;
}

#[allow(unused_variables)]
pub trait GraphicsBackend {
    type Output;
    type Error;

    fn get_output(&mut self, context: &Graphics) -> Result<Self::Output, Self::Error>;

    fn on_start(&mut self, context: &Graphics, state: &mut StyleResolver) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_finish(&mut self, context: &Graphics, state: &mut StyleResolver) -> Result<(), Self::Error> {
        Ok(())
    }

    fn draw(&mut self, context: &Graphics, state: &mut StyleResolver, drawable: &Drawable) -> Result<(), Self::Error> {
        match drawable {
            Drawable::Style(inner) => Ok(state.set_local_style(inner.clone())),
            Drawable::Shape(inner) if inner.is_empty(state) => Ok(()),
            Drawable::Shape(inner) => match inner {
                GraphicsShape::Point(s) => self.draw_point(context, state, s),
                GraphicsShape::Line(s) => self.draw_line(context, state, s),
                GraphicsShape::Pixel(s) => self.draw_pixel(context, state, s),
                GraphicsShape::Rectangle(s) => self.draw_rectangle(context, state, s),
            },
        }
    }
    fn draw_pixel(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Pixel) -> Result<(), Self::Error>;
    fn draw_point(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Point) -> Result<(), Self::Error>;
    fn draw_line(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Line) -> Result<(), Self::Error>;
    fn draw_rectangle(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Rectangle) -> Result<(), Self::Error>;
}
