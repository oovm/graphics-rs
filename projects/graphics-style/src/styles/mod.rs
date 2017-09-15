mod color;
pub use self::color::RGBA;
use palette::{Srgb, Srgba};
use std::{
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
    marker::PhantomData,
};
