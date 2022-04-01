mod serder;
use super::*;

///
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
