// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

use graphics_style::{GraphicsStyle, PointStyle, StyleResolver};

#[derive(Debug, Default)]
pub struct Graphics {
    pub graphic: Vec<Box<dyn Drawable2D>>,
    pub setting: GraphicsSetting,
    pub style: StyleResolver,
}

pub struct GraphicsSetting {}

pub enum GraphicsShape {
    Point(Point),
}

pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub struct StyledPoint {
    pub point: Point,
    pub style: PointStyle,
}

pub trait Drawable2D {
    fn get_shape(&self) -> Option<GraphicsShape>;
    fn get_style_effect(&self) -> Vec<&dyn GraphicsStyle> {
        vec![]
    }
    fn get_style_once(&self) -> Vec<&dyn GraphicsStyle> {
        vec![]
    }
}

impl Drawable2D for dyn GraphicsStyle {
    fn get_shape(&self) -> Option<GraphicsShape> {
        None
    }
    fn get_style_effect(&self) -> Vec<&dyn GraphicsStyle> {
        vec![self]
    }
    fn get_style_once(&self) -> Vec<&dyn GraphicsStyle> {
        vec![]
    }
}

impl Drawable2D for StyledPoint {
    fn get_shape(&self) -> Option<GraphicsShape> {
        Some(GraphicsShape::Point(self.point))
    }
    fn get_style_effect(&self) -> Vec<&dyn GraphicsStyle> {
        vec![&self.style]
    }
    fn get_style_once(&self) -> Vec<&dyn GraphicsStyle> {
        vec![&self.style]
    }
}

impl Drawable2D for Point {
    fn get_shape(&self) -> Option<GraphicsShape> {
        Some(GraphicsShape::Point(*self))
    }
    fn get_style_effect(&self) -> Vec<&dyn GraphicsStyle> {
        vec![]
    }
    fn get_style_once(&self) -> Vec<&dyn GraphicsStyle> {
        vec![]
    }
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
                GraphicsShape::Pixel(s) => self.draw_pixel(context, state, s),
                GraphicsShape::Line(s) => self.draw_line(context, state, s),
                GraphicsShape::Circle(s) => self.draw_rectangle(context, state, s),
                GraphicsShape::Rectangle(s) => self.draw_rectangle(context, state, s),
                GraphicsShape::Polygon(s) => self.draw_rectangle(context, state, s),
            },
        }
    }
    fn draw_pixel(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Pixel) -> Result<(), Self::Error>;
    fn draw_line(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Line) -> Result<(), Self::Error>;
    fn draw_circle(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Line) -> Result<(), Self::Error>;
    fn draw_rectangle(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Rectangle) -> Result<(), Self::Error> {
        self.draw_polygon(context, state, &Polygon::from(shape))
    }
    fn draw_polygon(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Polygon) -> Result<(), Self::Error>;
}
