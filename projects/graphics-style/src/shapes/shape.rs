use super::*;

/// Represent the available style of a point.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct PointStyle {
    /// Represent the size of a point, default size is 1.0, see more in [`PointSize`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_size: Option<PointSize>,
    /// Represent the color of a point, default color is black, see more in [`PointColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_color: Option<PointColor>,
}

/// Represent the size of a point, default size is 1.0
///
/// The shape of the point is always round.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct PointSize {
    /// Actual value for [`StyleResolver::point_size`]
    pub value: f32,
}

/// Represent the color of a point, default color is black
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct PointColor {
    /// Actual value for [`StyleResolver::point_color`]
    pub value: RGBA,
}

/// Represent the available style of a point.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct Point3DStyle {
    /// Represent the size of a point, default size is 1.0, see more in [`PointSize`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_size: Option<PointSize>,
    /// Represent the color of a point, default color is black, see more in [`PointColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_color: Option<PointColor>,
}

/// Represent the available style of a circle.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct CircleStyle {
    /// Represent the width of a circle, default width is 1.0, see more in [`CircleWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_width: Option<CircleWidth>,
    /// Represent the color of a point, default color is black, see more in [`CircleColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_color: Option<CircleColor>,
}

/// Represent the width of a circle, default width is 1.0
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct CircleWidth {
    /// Actual value for [`StyleResolver::circle_width`]
    pub value: f32,
}

/// Represent the color of a point, default color is black
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct CircleColor {
    /// Actual value for [`StyleResolver::circle_color`]
    pub value: RGBA,
}

/// Represent the available style of a disk.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DiskStyle {
    /// Represent the color of a disk, default color is black, see more in [`DiskFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_fill_color: Option<DiskFillColor>,
    /// Represent the edge width of a disk, default width is 1.0, see more in [`DiskEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_edge_width: Option<DiskEdgeWidth>,
    /// Represent the edge color of a disk, default is transparent, see more in [`DiskEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_edge_color: Option<DiskEdgeColor>,
}

/// Represent the color of a disk, default color is black
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct DiskFillColor {
    /// Actual value for [`StyleResolver::disk_fill_color`]
    pub value: RGBA,
}

/// Represent the edge width of a disk, default width is 1.0
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct DiskEdgeWidth {
    /// Actual value for [`StyleResolver::disk_edge_width`]
    pub value: f32,
}

/// Represent the edge color of a disk, default is transparent
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct DiskEdgeColor {
    /// Actual value for [`StyleResolver::disk_edge_color`]
    pub value: RGBA,
}

/// Represent the available style of a line.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LineStyle {
    /// Represent the with of a line, default width is 1.0, see more in [`LineWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,
    /// Represent the color of a line, default color is black, see more in [`LineColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<LineColor>,
}

/// Represent the with of a line, default width is 1.0
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct LineWidth {
    /// Actual value for [`StyleResolver::line_width`]
    pub value: f32,
}

/// Represent the color of a line, default color is black
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct LineColor {
    /// Actual value for [`StyleResolver::line_color`]
    pub value: RGBA,
}

/// Represent the available style of a triangle.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TriangleStyle {
    /// Represent the with of a line, default width is 1.0, see more in [`TriangleEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_edge_width: Option<TriangleEdgeWidth>,
    /// Represent the color of a line, default color is black, see more in [`TriangleFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_fill_color: Option<TriangleFillColor>,
}

/// Represent the with of a line, default width is 1.0
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct TriangleEdgeWidth {
    /// Actual value for [`StyleResolver::triangle_edge_width`]
    pub value: f32,
}

/// Represent the color of a line, default color is black
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct TriangleFillColor {
    /// Actual value for [`StyleResolver::triangle_fill_color`]
    pub value: RGBA,
}

/// Represent the available style of a triangle.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SquareStyle {
    /// Represent the with of a line, default width is 1.0, see more in [`SquareEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_edge_width: Option<SquareEdgeWidth>,
    /// Represent the color of a line, default color is black, see more in [`SquareFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_fill_color: Option<SquareFillColor>,
}

/// Represent the with of a line, default width is 1.0
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct SquareEdgeWidth {
    /// Actual value for [`StyleResolver::square_edge_width`]
    pub value: f32,
}

/// Represent the color of a line, default color is black
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct SquareFillColor {
    /// Actual value for [`StyleResolver::square_fill_color`]
    pub value: RGBA,
}

/// Represent the available style of a triangle.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RectangleStyle {
    /// Represent the with of a line, default width is 1.0, see more in [`SquareEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_edge_width: Option<SquareEdgeWidth>,
    /// Represent the color of a line, default color is black, see more in [`SquareFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_fill_color: Option<SquareFillColor>,
}
