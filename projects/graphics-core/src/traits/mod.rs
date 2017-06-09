use crate::{Drawable, Graphics, GraphicsShape, Line, Pixel, Point};

pub trait Distance {
    type Other;
    fn distance(&self, other: &Self::Other) -> f32;
}

#[allow(unused_variables)]
pub trait GraphicsBackend {
    type Output;
    type Error;

    fn get_output(&mut self, state: &Graphics) -> Result<Self::Output, Self::Error>;

    fn on_start(&mut self, state: &mut Graphics) -> Result<(), Self::Error> {
        Ok(())
    }
    fn on_finish(&mut self, state: &mut Graphics) -> Result<(), Self::Error> {
        Ok(())
    }

    fn draw(&mut self, state: &mut Graphics) -> Result<(), Self::Error> {
        for drawable in &state.graphic {
            match drawable {
                Drawable::Style(inner) => state.style.set_local_style(inner.clone()),
                Drawable::Shape(inner) => match inner {
                    GraphicsShape::Point(s) => self.draw_point(state, s)?,
                    GraphicsShape::Line(s) => self.draw_line(state, s)?,
                    GraphicsShape::Pixel(s) => self.draw_pixel(state, s)?,
                },
            }
        }
        Ok(())
    }
    fn draw_pixel(&mut self, context: &Graphics, shape: &Pixel) -> Result<(), Self::Error>;
    fn draw_point(&mut self, context: &Graphics, shape: &Point) -> Result<(), Self::Error>;
    fn draw_line(&mut self, context: &Graphics, shape: &Line) -> Result<(), Self::Error>;
}
