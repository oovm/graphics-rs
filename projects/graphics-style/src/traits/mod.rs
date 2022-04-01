mod add_assign;
mod convert;
mod draw_style;
use crate::{resolver::StyleContext, *};
use std::{cmp::Ordering, ops::AddAssign};

/// Trait for drawing a shape with a style.
#[allow(unused_variables)]
pub trait GraphicsStyle3D {
    /// Draws a shape with a style.
    fn skip(&self) -> bool {
        false
    }
    /// Draws a shape with a style.
    ///
    /// # Arguments
    ///
    /// * `state`:
    ///
    /// returns: ()
    ///
    /// # Examples
    ///
    /// ```rust
    /// use graphics_style::{Color, GraphicsStyle3D, StyleContext};
    /// pub struct CustomLineStyle {
    ///     pub width: f32,
    ///     pub color: Color,
    /// }
    ///
    /// impl GraphicsStyle3D for CustomLineStyle {
    ///     fn change_style(&self, state: &mut StyleContext) {
    ///         state.line_width = Some(self.width);
    ///         state.line_color = Some(self.color);
    ///     }
    /// }
    /// ```
    fn change_style(&mut self, state: &mut StyleContext);
}

impl GraphicsStyle3D for () {
    fn change_style(&self, _: &mut StyleContext) {}
}
