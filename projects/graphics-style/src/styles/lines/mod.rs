use super::*;
mod width;

/// Line width
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LineWidth {
    /// The line width
    pub value: f32,
}
