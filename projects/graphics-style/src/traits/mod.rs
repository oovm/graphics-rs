mod add_assign;
mod draw_style;
use crate::{resolver::StyleContext, *};
use std::ops::AddAssign;

/// Trait for drawing a shape with a style.
pub trait GraphicsStyle {
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
    /// use graphics_style::{GraphicsStyle, StyleContext, RGBA};
    /// pub struct CustomLineStyle {
    ///     pub width: f32,
    ///     pub color: RGBA,
    /// }
    ///
    /// impl GraphicsStyle for CustomLineStyle {
    ///     fn draw_style(&self, state: &mut StyleContext) {
    ///         state.line_width = Some(self.width);
    ///         state.line_color = Some(self.color);
    ///     }
    /// }
    /// ```
    fn draw_style(&self, state: &mut StyleContext);
}
