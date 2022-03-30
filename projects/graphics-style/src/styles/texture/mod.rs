use super::*;
mod transform;

pub use transform::{TextureFilter, TextureTransform, TextureWrap};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Texture {
    Color(Color),
    Image(Image),
    Gradient(Gradient),
}

impl From<Color> for Texture {
    fn from(color: Color) -> Self {
        Self::Color(color)
    }
}

impl From<Image> for Texture {
    fn from(image: Image) -> Self {
        Self::Image(image)
    }
}
impl From<Gradient> for Texture {
    fn from(gradient: Gradient) -> Self {
        Self::Gradient(gradient)
    }
}

impl Default for Texture {
    fn default() -> Self {
        Self::Color(Color::new(0, 0, 0, 0))
    }
}
