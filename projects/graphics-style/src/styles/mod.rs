mod color;
mod gradient;
mod settable;
mod tex_image;
mod texture;
pub use self::{color::Color, gradient::Gradient, tex_image::Image, texture::*};
use graphics_error::{parse_error, GraphicsError, Result};
use image::RgbaImage;
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::{
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
    str::FromStr,
    sync::Arc,
};
