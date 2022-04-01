use super::*;

impl Default for EdgeStyle {
    fn default() -> Self {
        Self { texture: Texture::Color(Color::BLACK), width: 1.0 }
    }
}

impl EdgeStyle {
    pub fn empty() -> Self {
        Self { texture: Texture::Color(Color::new(0, 0, 0, 0)), width: -1.0 }
    }
}
