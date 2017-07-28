#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

pub use self::{
    context::{GraphicsStyle, StyleResolver},
    styles::*,
};

mod context;
mod styles;
mod traits;
