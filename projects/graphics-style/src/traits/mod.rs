mod add_assign;
mod draw_style;
use crate::{resolver::StyleContext, *};
use std::ops::AddAssign;

pub trait GraphicsStyle {
    fn draw_style(&self, state: &mut StyleContext);
}
