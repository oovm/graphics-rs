use super::*;
mod builtin;
mod traits;

/// A color with red, green, blue, and alpha channel.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct RGBA {
    /// The red channel takes a value from 0 to 255.
    pub r: u8,
    /// The green channel takes a value from 0 to 255.
    pub g: u8,
    /// The blue channel takes a value from 0 to 255.
    pub b: u8,
    /// The alpha channel takes a value from 0 to 255.
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
