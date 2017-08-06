mod add_assign;
mod draw_style;
mod from;
use crate::{context::StyleContext, *};
use std::ops::AddAssign;

pub trait GraphicsStyle {
    fn draw_style(&self, state: &mut StyleContext);
}
