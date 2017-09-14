use palette::Srgba;
use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
};

mod circle;
mod color;
mod disk;
mod lines;
mod points;

pub use self::color::RGBA;

include!("shape.rs");
