use image::{ImageBuffer, RgbaImage};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]

pub struct Image {
    /// A reference to the image in srgb space.
    shared: Arc<RgbaImage>,
}

impl Image {
    pub fn width(&self) -> u32 {
        self.shared.width()
    }
    pub fn height(&self) -> u32 {
        self.shared.height()
    }
}
