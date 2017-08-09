// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

pub use self::{resolver::StyleResolver, styles::*, traits::GraphicsStyle};

mod resolver;
mod styles;
mod traits;
