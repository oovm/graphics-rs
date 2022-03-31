use super::*;
use graphics_shape::Ellipse;
use graphics_style::{
    resolved::{LineStyle, PolygonStyle, RectangleStyle},
    StyleContext,
};

#[allow(unused_variables)]
pub trait GraphicsBackend {
    type Output;
    type Error;

    fn get_output(&mut self, context: &Graphics) -> Result<Self::Output, Self::Error>;

    fn on_start(&mut self, context: &Graphics, state: &mut StyleContext) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_finish(&mut self, context: &Graphics, state: &mut StyleContext) -> Result<(), Self::Error> {
        Ok(())
    }

    fn draw_rectangle(&mut self, context: &Graphics, shape: &Rectangle, style: RectangleStyle) -> Result<(), Self::Error> {
        Ok(())
    }

    fn draw_ellipse(&mut self, context: &Graphics, shape: &Ellipse, style: &EllipseStyle) -> Result<(), Self::Error>;

    fn draw_polyline(&mut self, context: &Graphics, shape: &Line, style: &LineStyle) -> Result<(), Self::Error>;
    /// Draws any polygon like triangle .
    fn draw_polygon(&mut self, context: &Graphics, shape: &Polygon, style: &PolygonStyle) -> Result<(), Self::Error>;
}
