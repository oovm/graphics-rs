mod convert;
#[cfg(feature = "wolfram_wxf")]
mod wolfram;
use crate::{Drawable, Graphics, GraphicsShape, Line, Pixel, Polygon, Rectangle};
use graphics_style::{GraphicsStyle, PointStyle, StyleResolver};

pub trait Distance {
    type Other;
    fn distance(&self, other: &Self::Other) -> f32;
}

include!("backend.rs");
