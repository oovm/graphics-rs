use crate::SVG;
use graphics_core::{Graphics, GraphicsBackend, GraphicsError, Line, Pixel, Point, StyleResolver};
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
    type Error = GraphicsError;

    fn get_output(&mut self, _: &Graphics) -> Result<Self::Output, Self::Error> {
        let attributes = &[
            ("width", format!("{}", self.width)),
            ("height", format!("{}", self.height)),
            ("viewBox", format!("0 0 {} {}", self.width, self.height)),
            ("xmlns", "http://www.w3.org/2000/svg".to_string()),
        ];
        Ok(SVG::new("svg", attributes, take(&mut self.buffer)))
    }

    fn on_start(&mut self, context: &Graphics, _: &mut StyleResolver) -> Result<(), Self::Error> {
        self.width = context.setting.width;
        self.height = context.setting.height;
        Ok(())
    }

    fn draw_pixel(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Pixel) -> Result<(), Self::Error> {
        todo!()
    }

    fn draw_point(&mut self, _: &Graphics, state: &mut StyleResolver, shape: &Point) -> Result<(), Self::Error> {
        if shape.is_empty(state) {
            return Ok(());
        }
        let size = shape.get_size(state);
        let color = shape.get_color(state);
        let attributes = &[
            ("cx", format!("{}", shape.get_x())),
            ("cy", format!("{}", shape.get_y())),
            ("r", format!("{}", size.value)),
            ("fill", format!("{:#X}", color)),
        ];
        let point = SVG::new("circle", attributes, vec![]);
        Ok(self.buffer.push(point))
    }

    fn draw_line(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Line) -> Result<(), Self::Error> {
        if shape.is_empty(state) {
            return Ok(());
        }
        return Ok(());
    }
}
