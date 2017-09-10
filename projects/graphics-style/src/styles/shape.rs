use super::*;

/// Represent the available style of a point.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PointStyle {
    /// Represent the size of a point, see more in [`PointSize`].
    pub point_size: Option<f32>,
    /// , see more in [`PointColor`].
    pub point_color: Option<RGBA>,
}

/// Represent the size of a point
///
/// 1=1px on canvas.
///
/// The shape of the point is always round.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PointSize {
    /// Actual value for [`StyleResolver::point_size`]
    pub value: f32,
}

///

#[derive()]
pub struct PointColor {
    /// Actual value for [`StyleResolver::point_color`]
    pub value: RGBA,
}

///
#[derive()]
pub struct CircleStyle {
    /// , see more in [`CircleWidth`].
    pub circle_width: Option<f32>,
    /// , see more in [`CircleColor`].
    pub circle_color: Option<RGBA>,
}

///

#[derive()]
pub struct CircleWidth {
    /// Actual value for [`StyleResolver::circle_width`]
    pub value: f32,
}

///

#[derive()]
pub struct CircleColor {
    /// Actual value for [`StyleResolver::circle_color`]
    pub value: RGBA,
}

///
#[derive()]
pub struct DiskStyle {
    /// , see more in [`DiskFillColor`].
    pub disk_fill_color: Option<RGBA>,
    /// , see more in [`DiskEdgeWidth`].
    pub disk_edge_width: Option<f32>,
    /// , see more in [`DiskEdgeColor`].
    pub disk_edge_color: Option<RGBA>,
}

///

#[derive()]
pub struct DiskFillColor {
    /// Actual value for [`StyleResolver::disk_fill_color`]
    pub value: RGBA,
}

///

#[derive()]
pub struct DiskEdgeWidth {
    /// Actual value for [`StyleResolver::disk_edge_width`]
    pub value: f32,
}

///

#[derive()]
pub struct DiskEdgeColor {
    /// Actual value for [`StyleResolver::disk_edge_color`]
    pub value: RGBA,
}

///
#[derive()]
pub struct LineStyle {
    /// , see more in [`LineWidth`].
    pub line_width: Option<f32>,
    /// , see more in [`LineColor`].
    pub line_color: Option<RGBA>,
}

///

#[derive()]
pub struct LineWidth {
    /// Actual value for [`StyleResolver::line_width`]
    pub value: f32,
}

///

#[derive()]
pub struct LineColor {
    /// Actual value for [`StyleResolver::line_color`]
    pub value: RGBA,
}
