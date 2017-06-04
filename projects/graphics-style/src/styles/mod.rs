use palette::Srgba;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
};
mod color;
mod lines;
mod points;

pub use color::RGBA;
pub use lines::{LineColor, LineWidth};
pub use points::PointSize;
