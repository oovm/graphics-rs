mod color;
mod gradient;
mod image;
mod texture;
pub use self::{color::Color, gradient::Gradient, image::Image, texture::*};
use graphics_error::{parse_error, GraphicsError, Result};
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
    str::FromStr,
};
