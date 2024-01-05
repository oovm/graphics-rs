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
    Deserializer, Serializer,
};
use serde_derive::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter, LowerHex, UpperHex, Write},
    str::FromStr,
    sync::Arc,
};
use std::borrow::Cow;


/// The style of a graphic element.
#[derive(Debug)]
pub enum StyleAttribute<T> {
    /// Inherit the style from the parent.
    Inherit,
    /// Use the initial style.
    Initial,
    /// Use the given style.
    Custom(Box<T>),
    /// Use the given style.
    Shared(Arc<T>),
}

impl<T> Default for StyleAttribute<T> {
    fn default() -> Self {
        Self::Inherit
    }
}

impl<T: GraphicStyle> StyleAttribute<T> {
    fn get_style(&self, theme: &Option<Self>) -> Cow<Self> {
        match self {
            Self::Inherit => {
                match theme {
                    None => { Cow::Owned(T::default()) }
                    Some(s) => {
                        match s {
                            Self::Inherit => {}
                            Self::Initial => {}
                            Self::Custom(_) => {}
                            Self::Shared(_) => {}
                        }

                    }
                }
            }
            Self::Initial => {

            }
            Self::Custom(s) => {
                Cow::Borrowed(s)
            }
            Self::Shared(s) => {  Cow::Borrowed(s) }
        }
    }
}

pub trait GraphicStyle: Default {}
