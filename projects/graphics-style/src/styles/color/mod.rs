mod builtin;
mod display;
mod traits;

use super::*;

/// A color with red, green, blue, and alpha components.
#[derive(Debug, Copy, Clone)]
pub struct RGBA(Srgba);

impl RGBA {
    /// Creates a new RGBA color.
    pub const fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self(Srgba { color: Srgb { red, green, blue, standard: PhantomData }, alpha })
    }

    /// Creates a new RGBA color.
    pub fn view(&self) -> (u8, u8, u8, u8) {
        let r = self.0.red * 255.0;
        let g = self.0.green * 255.0;
        let b = self.0.blue * 255.0;
        let a = self.0.alpha * 255.0;
        (r as u8, g as u8, b as u8, a as u8)
    }
    /// Check if the color is opaque.
    pub fn is_empty(&self) -> bool {
        self.0.alpha <= 0.0
    }
}
