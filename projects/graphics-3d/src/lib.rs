// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

use graphics_style::{GraphicsStyle, PointStyle, StyleResolver};

#[derive(Debug, Default)]
pub struct Graphics<'g> {
    pub graphic: Vec<&'g dyn Drawable>,
    pub setting: GraphicsSetting,
    pub style: StyleResolver,
}

pub struct GraphicsSetting {}

pub enum GraphicsShape {
    Circle(Circle),
}

#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

pub struct StyledCircle {
    pub shape: Circle,
    pub style: PointStyle,
}

pub trait Drawable {
    fn get_shape(&self) -> Option<GraphicsShape>;
    fn changed_style(&self) -> Vec<&dyn GraphicsStyle> {
        vec![]
    }
    fn change_style_once(&self) -> Vec<&dyn GraphicsStyle>;
}

impl Drawable for &dyn GraphicsStyle {
    fn get_shape(&self) -> Option<GraphicsShape> {
        None
    }
    fn changed_style(&self) -> Vec<&dyn GraphicsStyle> {
        vec![self]
    }

    fn change_style_once(&self) -> Vec<&dyn GraphicsStyle> {
        vec![]
    }
}

impl Drawable for Circle {
    fn get_shape(&self) -> Option<GraphicsShape> {
        Some(GraphicsShape::Circle(*self))
    }
    fn change_style_once(&self) -> Vec<&dyn GraphicsStyle> {
        vec![]
    }
}

impl Drawable for StyledCircle {
    fn get_shape(&self) -> Option<GraphicsShape> {
        Some(GraphicsShape::Circle(self.shape))
    }
    fn change_style_once(&self) -> Vec<&dyn GraphicsStyle> {
        vec![&self.style]
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

    fn draw(&mut self, context: &Graphics, state: &mut StyleResolver, drawable: &dyn Drawable) -> Result<(), Self::Error> {
        state.set_local_style(&drawable.changed_style());
        state.set_once_style(&drawable.changed_style());

        let shape = match drawable.get_shape() {
            Some(GraphicsShape::Circle(s)) => {
                self.draw_circle(context, state, s.x, s.y, 1.0)?;
            }
            None => {}
        };

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

        Ok(state.clean_once_style())
    }
    fn draw_pixel(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Pixel) -> Result<(), Self::Error>;
    fn draw_line(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Line) -> Result<(), Self::Error>;
    fn draw_circle(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Line) -> Result<(), Self::Error>;
    fn draw_rectangle(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Rectangle) -> Result<(), Self::Error> {
        self.draw_polygon(context, state, &Polygon::from(shape))
    }
    fn draw_polygon(&mut self, context: &Graphics, state: &mut StyleResolver, shape: &Polygon) -> Result<(), Self::Error>;
}
