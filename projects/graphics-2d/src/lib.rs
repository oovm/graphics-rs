#![feature(generic_associated_types)]
// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

mod canvas;
mod elements;
mod macros;
mod traits;

pub use crate::{canvas::*, elements::*, macros::*, traits::*};

pub mod shapes {
    pub use graphics_shape::*;
}

pub mod styles {
    pub use graphics_style::*;
}
