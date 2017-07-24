mod convert;
#[cfg(feature = "wolfram_wxf")]
mod wolfram;
use crate::{Line, Point, Rectangle};
use graphics_style::StyleResolver;

pub trait Distance {
    type Other;
    fn distance(&self, other: &Self::Other) -> f32;
}
