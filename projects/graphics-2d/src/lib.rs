#![feature(generic_associated_types)]
// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

mod canvas;
mod elements;
mod macros;
mod traits;

pub use crate::{canvas::*, elements::*, macros::*, traits::*};

impl Clone for Drawable {
    fn clone(&self) -> Self {
        todo!()
    }
}

use graphics_shape::Circle;
use graphics_style::{CircleStyle, Color, GraphicsStyle};
use std::borrow::Cow;

#[derive(Debug)]
pub enum Drawable {
    StyleChange { style: Box<dyn GraphicsStyle>, finish: bool },
    Circle { shape: Circle, style: CircleStyle, finish: bool },
}

impl Drawable {
    pub fn change_style(&self) {}
}

impl Drawable {
    pub fn should_remove(&self) -> bool {
        *match self {
            Self::StyleChange { finish, .. } => finish,
            Self::Circle { finish, .. } => finish,
        }
    }
}

impl Graphics {
    pub fn gc(&mut self) {
        // drain_filter
        // let mut next_frame = Vec::with_capacity(self.graphic.len());
        // for i in self.graphic.into_iter() {
        //     if !i.should_remove() {
        //         next_frame.push(i);
        //     }
        // }
    }
}
