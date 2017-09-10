use super::*;

mod size;
/// Represent the available style of a point.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PointStyle {
    /// Represent the size of the point, see more in [`PointSize`].
    pub point_size: Option<f32>,
    /// Represent the color of the point, see more in [`PointColor`].
    pub point_color: Option<RGBA>,
}

/// Represent the size of a point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointSize {
    /// Represent 1px on canvas.
    ///
    /// Actual occupancy depends on shape.
    pub value: f32,
}

/// Represent the color of a point.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PointColor {
    /// The line width
    pub value: RGBA,
}
