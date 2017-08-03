mod draw_style;
mod upcast;
use crate::{context::StyleContext, *};
use std::ops::AddAssign;

pub trait GraphicsStyle {
    fn draw_style(&self, state: &mut StyleContext);
}
