use super::*;
use palette::{Gradient, Srgb};

pub enum Texture {
    Color(Srgb),
    Image(Image),
    Gradient(Gradient),
}

pub struct Image {}

impl From<Color> for Texture {
    fn from(color: Color) -> Self {
        Self::Color(color)
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self::Color(Color::new(0, 0, 0, 0))
    }
}
