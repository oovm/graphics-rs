// #![feature(generic_associated_types)]
// // #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
// #![doc = include_str!("../Readme.md")]
//
// mod canvas;
// mod elements;
// mod macros;
// mod traits;
//
// pub use crate::{canvas::*, elements::*, macros::*, traits::*};
// pub use graphics_style::{GraphicsStyle, StyleResolver};
// pub use traits::GraphicsBackend;
//
// #[derive(Debug)]
// pub enum Drawable {
//     Shape(GraphicsShape),
//     Style(Box<dyn GraphicsStyle>),
// }
//
// impl Clone for Drawable {
//     fn clone(&self) -> Self {
//         todo!()
//     }
// }

use graphics_style::GraphicsStyle;
use std::borrow::Cow;

pub struct CircleWidth {}

mod raw {
    pub struct Circle {}
}

pub struct Circle {
    shape: crate::raw::Circle,
    circle_width: Option<CircleWidth>,
}

pub enum Drawable<'s> {
    StyleChange { style: &'s dyn GraphicsStyle },
    Circle { shape: Cow<'s, crate::raw::Circle>, info: CircleInfo },
}

impl<'s> Drawable<'s> {
    pub fn should_remove(&self) -> bool {
        *match self {
            Self::StyleChange { finish, .. } => finish,
            Self::Circle { finish, .. } => finish,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CircleInfo {
    pub circle_width: f32,
    pub circle_texture: graphics_style::Texture,
    pub life_rest: Option<f32>,
}

#[derive(Debug, Default)]
pub struct GraphicsStateMachine<'a> {
    pub graphic: Vec<Drawable<'a>>,
    pub setting: GraphicsSetting,
    pub style: StyleResolver,
}

impl GraphicsStateMachine {
    pub fn gc(&mut self) {
        // drain_filter
        let mut next_frame = Vec::with_capacity(self.graphic.len());
        for i in self.graphic.into_iter() {
            if !i.should_remove() {
                next_frame.push(i);
            }
        }
    }
}
