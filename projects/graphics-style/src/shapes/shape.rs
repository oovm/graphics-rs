use super::*;

/// Represent the available style of a background.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackgroundStyle {
    /// Represent the color of a line, default color is black, see more in [`BackgroundColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<BackgroundColor>,
}

/// Represent the available style of a circle.
///
/// The circle is defined by its center and its radius.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CircleStyle {
    /// Represent the width of a circle, default width is 1.0, see more in [`CircleWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_width: Option<CircleWidth>,

    /// Represent the color of a point, default color is black, see more in [`CircleColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_color: Option<CircleColor>,
}

/// Represent the available style of a disk.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
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

/// Represent the available style of a style.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeColor {
    /// Represent the edge color of a disk, default is transparent, see more in [`DiskEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_edge_color: Option<DiskEdgeColor>,

    /// Represent the color of a line, default color is black, see more in [`TriangleEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_edge_color: Option<TriangleEdgeColor>,

    /// Represent the with of a line, default width is 1.0, see more in [`SquareEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_edge_color: Option<SquareEdgeColor>,

    /// Represent the color of a line, default color is black, see more in [`RectangleEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle_edge_color: Option<RectangleEdgeColor>,

    /// Represent the color of a line, default color is black, see more in [`PolygonEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_edge_color: Option<PolygonEdgeColor>,
}

/// Represent the available style of a style.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeStyle {
    /// Represent the edge width of a disk, default width is 1.0, see more in [`DiskEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_edge_width: Option<DiskEdgeWidth>,

    /// Represent the color of a line, default color is black, see more in [`TriangleEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_edge_color: Option<TriangleEdgeColor>,
}

/// Represent the available style of a style.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeWidth {
    /// Represent the edge width of a disk, default width is 1.0, see more in [`DiskEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_edge_width: Option<DiskEdgeWidth>,

    /// Represent the with of a line, default width is 1.0, see more in [`TriangleEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_edge_width: Option<TriangleEdgeWidth>,

    /// Represent the with of a line, default width is 1.0, see more in [`SquareEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_edge_width: Option<SquareEdgeWidth>,

    /// Represent the with of a line, default width is 1.0, see more in [`RectangleEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle_edge_width: Option<RectangleEdgeWidth>,

    /// Represent the with of a line, default width is 1.0, see more in [`PolygonEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_edge_width: Option<PolygonEdgeWidth>,
}

/// Represent the available style of a style.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FillColor {
    /// Represent the color of a disk, default color is black, see more in [`DiskFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_fill_color: Option<DiskFillColor>,

    /// Represent the color of a line, default color is black, see more in [`TriangleFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_fill_color: Option<TriangleFillColor>,

    /// Represent the color of a line, default color is black, see more in [`SquareFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_fill_color: Option<SquareFillColor>,

    /// Represent the color of a line, default color is black, see more in [`RectangleFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle_fill_color: Option<RectangleFillColor>,

    /// Represent the color of a line, default color is black, see more in [`PolygonFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_fill_color: Option<PolygonFillColor>,
}

/// Represent the available style of a line.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineStyle {
    /// Represent the with of a line, default width is 1.0, see more in [`LineWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Represent the color of a line, default color is black, see more in [`LineColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<LineColor>,
}

/// Represent the available style of a point.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PointStyle {
    /// Represent the size of a point, default size is 1.0, see more in [`PointSize`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_size: Option<PointSize>,

    /// Represent the color of a point, default color is black, see more in [`PointColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_color: Option<PointColor>,
}

/// Represent the available style of a polygon.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolygonStyle {
    /// Represent the color of a line, default color is black, see more in [`PolygonFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_fill_color: Option<PolygonFillColor>,

    /// Represent the with of a line, default width is 1.0, see more in [`PolygonEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_edge_width: Option<PolygonEdgeWidth>,

    /// Represent the color of a line, default color is black, see more in [`PolygonEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_edge_color: Option<PolygonEdgeColor>,
}

/// Represent the available style of a line.
///
/// The polyline is a collection of points.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolylineStyle {
    /// Represent the with of a line, default width is 1.0, see more in [`PolylineWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline_width: Option<PolylineWidth>,

    /// Represent the color of a line, default color is black, see more in [`PolylineColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline_color: Option<PolylineColor>,
}

/// Represent the available style of a rectangle.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RectangleStyle {
    /// Represent the color of a line, default color is black, see more in [`RectangleFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle_fill_color: Option<RectangleFillColor>,

    /// Represent the with of a line, default width is 1.0, see more in [`RectangleEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle_edge_width: Option<RectangleEdgeWidth>,

    /// Represent the color of a line, default color is black, see more in [`RectangleEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle_edge_color: Option<RectangleEdgeColor>,
}

/// Represent the available style of a triangle.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SquareStyle {
    /// Represent the color of a line, default color is black, see more in [`SquareFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_fill_color: Option<SquareFillColor>,

    /// Represent the with of a line, default width is 1.0, see more in [`SquareEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_edge_width: Option<SquareEdgeWidth>,

    /// Represent the with of a line, default width is 1.0, see more in [`SquareEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_edge_color: Option<SquareEdgeColor>,
}

/// Represent the available style of a text.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextStyle {
    /// Represent the color of a line, default color is black, see more in [`TextColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<TextColor>,

    /// Represent the with of a line, default width is 1.0, see more in [`TextSize`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_size: Option<TextSize>,

    /// Represent the color of a line, default color is black, see more in [`TextFont`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_font: Option<TextFont>,
}

/// Represent the available style of a triangle.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriangleStyle {
    /// Represent the color of a line, default color is black, see more in [`TriangleFillColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_fill_color: Option<TriangleFillColor>,

    /// Represent the with of a line, default width is 1.0, see more in [`TriangleEdgeWidth`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_edge_width: Option<TriangleEdgeWidth>,

    /// Represent the color of a line, default color is black, see more in [`TriangleEdgeColor`].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_edge_color: Option<TriangleEdgeColor>,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct BackgroundColor {
    /// Actual value for [`StyleResolver::background_color`]
    pub value: RGBA,
}

/// Represent the color of a point, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct CircleColor {
    /// Actual value for [`StyleResolver::circle_color`]
    pub value: RGBA,
}

/// Represent the width of a circle, default width is 1.0
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct CircleWidth {
    /// Actual value for [`StyleResolver::circle_width`]
    pub value: f32,
}

/// Represent the edge color of a disk, default is transparent
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct DiskEdgeColor {
    /// Actual value for [`StyleResolver::disk_edge_color`]
    pub value: RGBA,
}

/// Represent the edge width of a disk, default width is 1.0
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct DiskEdgeWidth {
    /// Actual value for [`StyleResolver::disk_edge_width`]
    pub value: f32,
}

/// Represent the color of a disk, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct DiskFillColor {
    /// Actual value for [`StyleResolver::disk_fill_color`]
    pub value: RGBA,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct LineColor {
    /// Actual value for [`StyleResolver::line_color`]
    pub value: RGBA,
}

/// Represent the with of a line, default width is 1.0
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct LineWidth {
    /// Actual value for [`StyleResolver::line_width`]
    pub value: f32,
}

/// Represent the color of a point, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct PointColor {
    /// Actual value for [`StyleResolver::point_color`]
    pub value: RGBA,
}

/// Represent the size of a point, default size is 1.0
///
/// The shape of the point is always round.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct PointSize {
    /// Actual value for [`StyleResolver::point_size`]
    pub value: f32,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct PolygonEdgeColor {
    /// Actual value for [`StyleResolver::polygon_edge_color`]
    pub value: RGBA,
}

/// Represent the with of a line, default width is 1.0
#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct PolygonEdgeWidth {
    /// Actual value for [`StyleResolver::polygon_edge_width`]
    pub value: f32,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct PolygonFillColor {
    /// Actual value for [`StyleResolver::polygon_fill_color`]
    pub value: RGBA,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct PolylineColor {
    /// Actual value for [`StyleResolver::polyline_color`]
    pub value: RGBA,
}

/// Represent the with of a line, default width is 1.0
#[derive(Clone, Default, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct PolylineWidth {
    /// Actual value for [`StyleResolver::polyline_width`]
    pub value: f32,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct RectangleEdgeColor {
    /// Actual value for [`StyleResolver::rectangle_edge_color`]
    pub value: RGBA,
}

/// Represent the with of a line, default width is 1.0
#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct RectangleEdgeWidth {
    /// Actual value for [`StyleResolver::rectangle_edge_width`]
    pub value: f32,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct RectangleFillColor {
    /// Actual value for [`StyleResolver::rectangle_fill_color`]
    pub value: RGBA,
}

/// Represent the with of a line, default width is 1.0
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct SquareEdgeColor {
    /// Actual value for [`StyleResolver::square_edge_color`]
    pub value: RGBA,
}

/// Represent the with of a line, default width is 1.0
#[derive(Clone, Copy, Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct SquareEdgeWidth {
    /// Actual value for [`StyleResolver::square_edge_width`]
    pub value: f32,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct SquareFillColor {
    /// Actual value for [`StyleResolver::square_fill_color`]
    pub value: RGBA,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct TextColor {
    /// Actual value for [`StyleResolver::text_color`]
    pub value: RGBA,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct TextFont {
    /// Actual value for [`StyleResolver::text_font`]
    pub value: f32,
}

/// Represent the with of a line, default width is 1.0
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct TextSize {
    /// Actual value for [`StyleResolver::text_size`]
    pub value: f32,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct TriangleEdgeColor {
    /// Actual value for [`StyleResolver::triangle_edge_color`]
    pub value: RGBA,
}

/// Represent the with of a line, default width is 1.0
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "f32", from = "f32")]
pub struct TriangleEdgeWidth {
    /// Actual value for [`StyleResolver::triangle_edge_width`]
    pub value: f32,
}

/// Represent the color of a line, default color is black
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(into = "RGBA", from = "RGBA")]
pub struct TriangleFillColor {
    /// Actual value for [`StyleResolver::triangle_fill_color`]
    pub value: RGBA,
}
