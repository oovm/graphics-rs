use crate::{
    canvas::Graphics,
    shapes::{Ellipse, Line, Polygon, Rectangle},
    styles::{EdgeStyle, EllipseStyle, PolygonStyle, RectangleStyle},
    GraphicsSetting,
};

#[allow(unused_variables)]
pub trait GraphicsBackend {
    type Output;
    type Error;

    fn get_output(&mut self, context: &Graphics) -> Result<Self::Output, Self::Error>;

    fn on_start(&mut self, state: &mut Graphics) -> Result<(), Self::Error> {
        Ok(())
    }

    fn on_finish(&mut self, state: &mut Graphics) -> Result<(), Self::Error> {
        Ok(())
    }

    ///
    #[rustfmt::skip]
    fn draw_rectangle(&mut self, context: &GraphicsSetting, shape: &Rectangle, style: &RectangleStyle) -> Result<(), Self::Error> {
        let shape = Polygon::from(shape);
        let style = PolygonStyle::from(style);
        self.draw_polygon(context, &shape, &style)
    }

    fn draw_ellipse(&mut self, context: &GraphicsSetting, shape: &Ellipse, style: &EllipseStyle) -> Result<(), Self::Error> {
        const N: usize = 48;
        let shape = shape.sample_polygon(N);
        let style = PolygonStyle::from(style);
        self.draw_polygon(context, &shape, &style)
    }

    fn draw_polyline(&mut self, context: &GraphicsSetting, shape: &Line, style: &EdgeStyle) -> Result<(), Self::Error>;
    /// Draws any polygon like triangle .
    fn draw_polygon(&mut self, context: &GraphicsSetting, shape: &Polygon, style: &PolygonStyle) -> Result<(), Self::Error>;
}
