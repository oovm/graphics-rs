use super::*;

/// Represent the available style of a point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiskStyle {
    /// Represent the size of the point, see more in [`PointSize`].
    pub disk_size: Option<f32>,
    /// Represent the color of the point, see more in [`PointColor`].
    pub disk_color: Option<RGBA>,
}

/// Represent the size of a point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DiskSize {
    /// Represent 1px on canvas.
    ///
    /// Actual occupancy depends on shape.
    pub value: f32,
}

/// Represent the color of a point.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DiskColor {
    /// The line width
    pub value: RGBA,
}
