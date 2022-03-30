use super::*;
use crate::styles::image::Image;
use palette::{Gradient, Srgb};

pub struct Texture {
    pub kind: TextureKind,
    pub wrap: TextureWrap,
    pub filter: TextureFilter,
}

pub enum TextureKind {
    Color(Color),
    Image(Image),
    Gradient(Gradient),
}

pub enum TextureWrap {
    Repeat,
    Clamp,
}

pub enum TextureFilter {
    Nearest,
    Linear,
}

pub impl From<Color> for TextureKind {
    fn from(color: Color) -> Self {
        Self::Color(color)
    }
}

impl Default for TextureKind {
    fn default() -> Self {
        Self::Color(Color::new(0, 0, 0, 0))
    }
}
