use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct StyleContext {
    pub circle_width: f32,
    pub circle_texture: Texture,
}

impl Default for StyleContext {
    fn default() -> Self {
        Self { circle_width: 1.0, circle_texture: Default::default() }
    }
}
