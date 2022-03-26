mod color;
pub use self::color::RGBA;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
    marker::PhantomData,
};
