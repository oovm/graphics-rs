use crate::*;
use graphics_style::CircleSize;

macro_rules! from_shape {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Drawable {
                fn from(v: $t) -> Self {
                    Self::Shape(v.into())
                }
            }
        )*
    };
}

macro_rules! from_style {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Drawable {
                fn from(v: $t) -> Self {
                    Self::Style(v.into())
                }
            }
        )*
    };
}

from_shape![Rectangle, Point];

from_style![CircleSize];
