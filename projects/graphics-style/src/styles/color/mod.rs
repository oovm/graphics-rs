use super::*;
use palette::{Srgb, Srgba};
use std::marker::PhantomData;
mod builtin;
mod convert;
mod display;
mod serder;

/// A color with red, green, blue, and alpha channel.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Color(Srgba<u8>);

impl Color {
    /// Creates a new RGBA color.
    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self(Srgba { color: Srgb { red, green, blue, standard: PhantomData }, alpha })
    }
    /// Creates a new RGBA color.
    #[inline(always)]
    pub const fn view(&self) -> (u8, u8, u8, u8) {
        (self.0.color.red, self.0.color.green, self.0.color.blue, self.0.alpha)
    }
}
