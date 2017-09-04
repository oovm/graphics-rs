use super::*;

/// Represent the available style of a point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleStyle {
    /// Represent the size of the point, see more in [`PointSize`].
    pub circle_width: Option<f32>,
    /// Represent the color of the point, see more in [`PointColor`].
    pub circle_color: Option<RGBA>,
}

/// Represent the size of a point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CircleWidth {
    /// Represent 1px on canvas.
    ///
    /// Actual occupancy depends on shape.
    pub value: f32,
}

/// Represent the color of a point.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct CircleColor {
    /// The line width
    pub value: RGBA,
}
