use super::*;
mod size;

/// A point style.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointSize {
    /// The size of the point.
    pub value: f32,
}
