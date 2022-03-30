mod color;
mod texture;
pub use self::{color::Color, texture::Texture};
use serde::{
    de::{MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::fmt::{Display, Formatter, LowerHex, UpperHex, Write};
