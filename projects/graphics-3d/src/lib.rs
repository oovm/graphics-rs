// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

use graphics_style::{CircleStyle, GraphicsStyle, PointStyle, StyleResolver};
use std::borrow::Cow;

pub struct Graphics<'g> {
    pub graphic: Vec<&'g dyn Drawable<Style = ()>>,
    pub setting: GraphicsSetting,
    pub style: StyleResolver,
}

pub struct GraphicsSetting {}

pub enum GraphicsShape<'s> {
    Circle(Cow<'s, RawCircle>),
}

#[derive(Debug, Copy, Clone)]
pub struct RawCircle {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

pub struct Circle {
    shape: RawCircle,
    style: CircleStyle,
}

pub trait Drawable {
    type Shape;
    type Style: GraphicsStyle;
    /// Draw the object
    fn get_shape(&self) -> Option<GraphicsShape>;
    /// Change default style
    fn get_style(&self) -> Cow<Self::Style>
    where
        <Self as Drawable>::Style: Clone;

    /// Change default style perming to change the style of the shape
    fn changed_style(&self) -> Vec<&dyn GraphicsStyle> {
        vec![]
    }
    fn skip(&self) -> bool {
        false
    }
}

impl Drawable for dyn GraphicsStyle {
    type Shape = ();
    type Style = ();

    fn get_shape(&self) -> Option<GraphicsShape> {
        None
    }
    /// It was always skipped because [Drawable::skip] is always false
    fn get_style(&self) -> Cow<Self::Style> {
        Cow::Owned(())
    }

    fn changed_style(&self) -> Vec<&dyn GraphicsStyle> {
        vec![self]
    }
    /// no need to draw, always skip
    fn skip(&self) -> bool {
        true
    }
}

impl Drawable for RawCircle {
    type Style = CircleStyle;

    fn get_shape(&self) -> Option<GraphicsShape> {
        Some(GraphicsShape::Circle(Cow::Borrowed(self)))
    }

    fn get_style(&self) -> Cow<Self::Style> {
        Cow::Owned(Self::Style::default())
    }
}

impl Drawable for Circle {
    type Style = CircleStyle;

    fn get_shape(&self) -> Option<GraphicsShape> {
        Some(GraphicsShape::Circle(Cow::Borrowed(&self.shape)))
    }

    fn get_style(&self) -> Cow<Self::Style> {
        Cow::Borrowed(&self.style)
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

    fn draw(
        &mut self,
        context: &Graphics,
        state: &mut StyleResolver,
        drawable: &dyn Drawable<Style = ()>,
    ) -> Result<(), Self::Error> {
        for i in drawable.changed_style() {
            state.set_local_style(i);
        }
        if drawable.skip() {
            return Ok(());
        }
        let shape = match drawable.get_shape() {
            Some(s) => s,
            None => return Ok(()),
        };
        drawable.get_style();
        match shape {
            GraphicsShape::Circle(c) => {
                let style = drawable.get_style();
                self.draw_circle(context, &c, style)
            }
        }
    }
    fn draw_circle(&mut self, context: &Graphics, shape: &RawCircle, style: &CircleStyle) -> Result<(), Self::Error>;
}
