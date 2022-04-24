use crate::SVG;
use graphics_2d::{
    shapes::{Ellipse, Line, Polygon, Rectangle},
    styles::{EdgeStyle, EllipseStyle, PolygonStyle, RectangleStyle},
    Graphics, GraphicsBackend, GraphicsSetting,
};
use shape_core::Line;
use std::mem::take;
use wolfram_expr::Expr;

pub struct WolframRenderer {
    buffer: Vec<Expr>,
}

impl Default for WolframRenderer {
    fn default() -> Self {
        Self { buffer: Vec::new() }
    }
}

impl GraphicsBackend for WolframRenderer {
    type Output = Expr;
    type Error = String;

    fn get_output(&mut self, context: &Graphics) -> Result<Self::Output, Self::Error> {
        todo!()
    }

    fn on_start(&mut self, state: &mut Graphics) -> Result<(), Self::Error> {
        todo!()
    }

    fn draw_polyline(&mut self, context: &GraphicsSetting, shape: &Line, style: &EdgeStyle) -> Result<(), Self::Error> {
        wl_symbol("Line", vec![]);
        Ok(())
    }

    fn draw_polygon(&mut self, context: &GraphicsSetting, shape: &Polygon, style: &PolygonStyle) -> Result<(), Self::Error> {
        wl_symbol("Polygon", vec![]);
        Ok(())
    }
}

fn wl_symbol(name: &str, terms: Vec<Expr>) -> Expr {
    let head = Expr::symbol(format!("System`{}", name));
    Expr::normal(head, terms)
}
