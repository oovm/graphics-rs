use super::*;

/// ```rust
/// text.transform(|t| t.scale(0.5, 0.5)).transform(|t| t.translate(0.5, 0.5)).finish();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub enum TextureWrap {
    Repeat,
    Clamp,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]

pub enum TextureFilter {
    Nearest,
    Linear,
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
