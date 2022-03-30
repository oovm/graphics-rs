use super::*;
use projective::{Projective, Projective3D};

/// ```rust
/// text.transform(|t| t.scale(0.5, 0.5)).transform(|t| t.translate(0.5, 0.5)).finish();
/// ```
pub struct TextureTransform {
    raw: Texture,
    wrap: TextureWrap,
    filter: TextureFilter,
}

impl Texture {
    pub fn transform(&self) -> TextureTransform {
        TextureTransform { raw: self.clone(), wrap: Default::default(), filter: Default::default() }
    }
}

impl TextureTransform {
    pub fn finish(self) -> Texture {
        todo!()
    }
    pub fn with_wrap(self, wrap: TextureWrap) -> Self {
        Self { wrap, ..self }
    }
    pub fn with_filter(self, filter: TextureFilter) -> Self {
        Self { filter, ..self }
    }
}

pub enum TextureWrap {
    Repeat,
    Clamp,
}

pub enum TextureFilter {
    Nearest,
    Linear,
}

impl Projective<f32> for TextureTransform {
    fn transform(&self, matrix: &[&f32; 9]) -> Self {
        let _ = matrix;
        todo!()
    }
}
impl<T> From<T> for Texture
where
    T: Into<Texture>,
{
    fn from(tex: T) -> Self {
        Self { kind: tex.into(), wrap: TextureWrap::Repeat, filter: TextureFilter::Nearest }
    }
}

impl Default for TextureWrap {
    fn default() -> Self {
        Self::Repeat
    }
}

impl Default for TextureFilter {
    fn default() -> Self {
        Self::Nearest
    }
}
