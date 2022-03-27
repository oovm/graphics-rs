use super::*;

/// Get default style when not specified.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StyleContext {
    /// Get default [`BackgroundColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<BackgroundColor>,

    /// Get default [`CircleColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_color: Option<CircleColor>,

    /// Get default [`CircleWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_width: Option<CircleWidth>,

    /// Get default [`DiskEdgeColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_edge_color: Option<DiskEdgeColor>,

    /// Get default [`DiskEdgeWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_edge_width: Option<DiskEdgeWidth>,

    /// Get default [`DiskFillColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_fill_color: Option<DiskFillColor>,

    /// Get default [`LineColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<LineColor>,

    /// Get default [`LineWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Get default [`PointColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_color: Option<PointColor>,

    /// Get default [`PointSize`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_size: Option<PointSize>,

    /// Get default [`PolygonEdgeColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_edge_color: Option<PolygonEdgeColor>,

    /// Get default [`PolygonEdgeWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_edge_width: Option<PolygonEdgeWidth>,

    /// Get default [`PolygonFillColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polygon_fill_color: Option<PolygonFillColor>,

    /// Get default [`PolylineColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline_color: Option<PolylineColor>,

    /// Get default [`PolylineWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub polyline_width: Option<PolylineWidth>,

    /// Get default [`RectangleEdgeColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle_edge_color: Option<RectangleEdgeColor>,

    /// Get default [`RectangleEdgeWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle_edge_width: Option<RectangleEdgeWidth>,

    /// Get default [`RectangleFillColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rectangle_fill_color: Option<RectangleFillColor>,

    /// Get default [`SquareEdgeColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_edge_color: Option<SquareEdgeColor>,

    /// Get default [`SquareEdgeWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_edge_width: Option<SquareEdgeWidth>,

    /// Get default [`SquareFillColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_fill_color: Option<SquareFillColor>,

    /// Get default [`TextColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<TextColor>,

    /// Get default [`TextFont`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_font: Option<TextFont>,

    /// Get default [`TextSize`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_size: Option<TextSize>,

    /// Get default [`TriangleEdgeColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_edge_color: Option<TriangleEdgeColor>,

    /// Get default [`TriangleEdgeWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_edge_width: Option<TriangleEdgeWidth>,

    /// Get default [`TriangleFillColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_fill_color: Option<TriangleFillColor>,
}
