mod builtin;
mod display;
mod traits;

use super::*;

/// A color with red, green, blue, and alpha channel.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RGBA {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGBA {
    /// Creates a new RGBA color.
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self { r: red, g: green, b: blue, a: alpha }
    }

    /// Check if the color is opaque.
    pub fn is_empty(&self) -> bool {
        self.a == 0
    }
}
