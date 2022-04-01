// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

use graphics_style::{GraphicsStyle3D, Texture};
use std::borrow::Cow;
mod canvas;
mod styles;
mod traits;
pub use canvas::*;
pub use traits::*;

mod raw {
    pub use graphics_shape::{Circle, Point};
}

pub enum GraphicsShape<'t> {
    Circle(Cow<'t, crate::raw::Circle>, CircleStyle),
}

pub struct Circle {
    shape: crate::raw::Circle,
    style: CircleStyle,
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CircleStyle {
    pub circle_width: f64,
    pub disk_texture: Texture,
}

pub trait Drawable {
    fn skip(&self) -> bool {
        false
    }
    /// Draw the object
    fn get_shape(&self) -> Option<GraphicsShape>;
    /// Change default style perming to change the style of the shape
    fn changed_style(&self) -> Vec<&dyn GraphicsStyle3D> {
        vec![]
    }
}

impl Drawable for dyn GraphicsStyle3D {
    fn get_shape(&self) -> Option<GraphicsShape> {
        None
    }
    /// It was always skipped because [Drawable::skip] is always false
    fn changed_style(&self) -> Vec<&dyn GraphicsStyle3D> {
        vec![self]
    }
    /// no need to draw, always skip
    fn skip(&self) -> bool {
        true
    }
}

impl Drawable for crate::raw::Circle {
    fn get_shape(&self) -> Option<GraphicsShape> {
        Some(GraphicsShape::Circle(Cow::Borrowed(self), Default::default()))
    }
}

impl Drawable for Circle {
    fn get_shape(&self) -> Option<GraphicsShape> {
        Some(GraphicsShape::Circle(Cow::Borrowed(&self.shape), self.style))
    }
}
