use crate::SVG;
use graphics_2d::{
    shapes::{Ellipse, Line, Polygon},
    styles::{EdgeStyle, EllipseStyle, PolygonStyle},
    Graphics, GraphicsBackend, GraphicsSetting,
};
use std::mem::take;

pub struct SvgRenderer {
    width: f32,
    height: f32,
    buffer: Vec<SVG>,
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self { width: 100.0, height: 100.0, buffer: Vec::new() }
    }
}

impl GraphicsBackend for SvgRenderer {
    type Output = SVG;
    type Error = String;

    fn get_output(&mut self, _: &Graphics) -> Result<Self::Output, Self::Error> {
        let attributes = &[
            ("width", format!("{}", self.width)),
            ("height", format!("{}", self.height)),
            ("viewBox", format!("0 0 {} {}", self.width, self.height)),
            ("xmlns", "http://www.w3.org/2000/svg".to_string()),
        ];
        Ok(SVG::new("svg", attributes, take(&mut self.buffer)))
    }

    fn draw_ellipse(&mut self, context: &GraphicsSetting, shape: &Ellipse, style: &EllipseStyle) -> Result<(), Self::Error> {
        todo!()
    }

    fn draw_polyline(&mut self, context: &GraphicsSetting, shape: &Line, style: &EdgeStyle) -> Result<(), Self::Error> {
        todo!()
    }

    fn draw_polygon(&mut self, context: &GraphicsSetting, shape: &Polygon, style: &PolygonStyle) -> Result<(), Self::Error> {
        todo!()
    }
}
