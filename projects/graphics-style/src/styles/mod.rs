use crate::context::StyleContext;
use palette::Srgba;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
    ops::{Add, AddAssign},
};
mod color;
mod lines;
mod points;
mod shape;

pub use color::RGBA;
pub use lines::{LineColor, LineWidth};
pub use points::{PointColor, PointSize};
