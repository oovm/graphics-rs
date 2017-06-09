use crate::SVG;
use graphics_core::{Graphics, GraphicsBackend, GraphicsError, Line, Pixel, Point, StyleResolver};
use std::{collections::BTreeMap, mem::take};

pub struct SvgRenderer {
    width: f32,
    height: f32,
    buffer: Vec<SVG>,
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

    fn on_start(&mut self, state: &mut Graphics) -> Result<(), Self::Error> {
        self.width = state.setting.width;
        self.height = state.setting.height;
        Ok(())
    }

    fn draw_pixel(&mut self, context: &Graphics, shape: &Pixel) -> Result<(), Self::Error> {
        todo!()
    }

    fn draw_point(&mut self, context: &Graphics, shape: &Point) -> Result<(), Self::Error> {
        let style = &context.style;
        if shape.is_empty(style) {
            return Ok(());
        }
        let size = shape.get_size(style);
        let color = shape.get_color(style);
        let attributes = &[
            ("cx", format!("{}", shape.get_x())),
            ("cy", format!("{}", shape.get_y())),
            ("r", format!("{}", size.value)),
            ("fill", format!("{:#X}", color)),
        ];
        let point = SVG::new("circle", attributes, take(&mut self.buffer));
        Ok(self.buffer.push(point))
    }

    fn draw_line(&mut self, context: &Graphics, shape: &Line) -> Result<(), Self::Error> {
        let style = &context.style;
        if shape.is_empty(style) {
            return Ok(());
        }
        return Ok(());
    }
}
