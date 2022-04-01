use crate::StyleContext;
use std::fmt::{Debug, Formatter};

/// Trait for drawing a shape with a style.
#[allow(unused_variables)]
pub trait GraphicsStyle {
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

impl Debug for dyn GraphicsStyle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "GraphicsStyle")
    }
}
