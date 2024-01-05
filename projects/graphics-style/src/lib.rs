#![deny(rustdoc::missing_crate_level_docs)]
// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/11549616")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/11549616")]

mod resolver;
mod shapes;
mod styles;
mod traits;

pub use crate::resolver::*;
pub use crate::shapes::*;
pub use crate::styles::*;
pub use crate::traits::*;
