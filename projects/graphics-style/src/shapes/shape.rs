/// Represent the available style of a point.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct PointStyle {
    /// Represent the size of a point, see more in [`PointSize`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_size: Option<PointSize>,
    /// , see more in [`PointColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_color: Option<PointColor>,
}

/// Represent the size of a point
///
/// 1=1px on canvas.
///
/// The shape of the point is always round.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct PointSize {
    /// Actual value for [`StyleResolver::point_size`]
    pub value: f32,
}

/// 1=1px on canvas.
///
/// The shape of the point is always round.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct PointColor {
    /// Actual value for [`StyleResolver::point_color`]
    pub value: RGBA,
}

///
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CircleStyle {
    /// , see more in [`CircleWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_width: Option<CircleWidth>,
    /// , see more in [`CircleColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_color: Option<CircleColor>,
}

/// 1=1px on canvas.
///
/// The shape of the point is always round.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct CircleWidth {
    /// Actual value for [`StyleResolver::circle_width`]
    pub value: f32,
}

/// 1=1px on canvas.
///
/// The shape of the point is always round.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct CircleColor {
    /// Actual value for [`StyleResolver::circle_color`]
    pub value: RGBA,
}

///
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DiskStyle {
    /// , see more in [`DiskFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_fill_color: Option<DiskFillColor>,
    /// , see more in [`DiskEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_edge_width: Option<DiskEdgeWidth>,
    /// , see more in [`DiskEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_edge_color: Option<DiskEdgeColor>,
}

/// 1=1px on canvas.
///
/// The shape of the point is always round.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct DiskFillColor {
    /// Actual value for [`StyleResolver::disk_fill_color`]
    pub value: RGBA,
}

/// 1=1px on canvas.
///
/// The shape of the point is always round.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct DiskEdgeWidth {
    /// Actual value for [`StyleResolver::disk_edge_width`]
    pub value: f32,
}

/// 1=1px on canvas.
///
/// The shape of the point is always round.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct DiskEdgeColor {
    /// Actual value for [`StyleResolver::disk_edge_color`]
    pub value: RGBA,
}

///
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LineStyle {
    /// , see more in [`LineWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,
    /// , see more in [`LineColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<LineColor>,
}

/// 1=1px on canvas.
///
/// The shape of the point is always round.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct LineWidth {
    /// Actual value for [`StyleResolver::line_width`]
    pub value: f32,
}

/// 1=1px on canvas.
///
/// The shape of the point is always round.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct LineColor {
    /// Actual value for [`StyleResolver::line_color`]
    pub value: RGBA,
}
