use crate::SVG;
use graphics_2d::{
    shapes::{Ellipse, Line, Polygon, Rectangle},
    styles::{EdgeStyle, EllipseStyle, PolygonStyle, RectangleStyle},
    Graphics, GraphicsBackend, GraphicsSetting,
};
use image::RgbaImage;
use std::mem::take;

pub struct ImageRenderer {
    width: f32,
    height: f32,
    buffer: RgbaImage,
}

impl Default for ImageRenderer {
    fn default() -> Self {
        Self { width: 100.0, height: 100.0, buffer: RgbaImage::new(100, 100) }
    }
}

impl GraphicsBackend for ImageRenderer {
    type Output = RgbaImage;
    type Error = String;

    fn get_output(&mut self, _: &Graphics) -> Result<Self::Output, Self::Error> {
        Ok(self.buffer.clone())
    }

    fn on_start(&mut self, state: &mut Graphics) -> Result<(), Self::Error> {
        todo!()
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
