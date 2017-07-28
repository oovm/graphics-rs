use palette::Srgba;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
};
mod color;
mod lines;
mod points;
mod shape;

pub use color::RGBA;
pub use lines::{LineColor, LineStyle, LineWidth};
pub use points::{PointColor, PointSize, PointStyle};
pub use shape::StyledShape;
