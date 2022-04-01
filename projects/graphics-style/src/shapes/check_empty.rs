use super::*;

impl EdgeStyle {
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.width <= 0.0 && self.texture.is_empty()
    }
}

impl FillStyle {
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.texture.is_empty()
    }
}

impl Texture {
    pub fn is_empty(&self) -> bool {
        match self {
            Texture::Color(c) => c.is_empty(),
            Texture::Image(_) => {
                todo!()
            }
            Texture::Gradient(_) => {
                todo!()
            }
        }
    }
}

impl Color {
    /// Check if the color is opaque.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.view().3 == 0
    }
}
