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

impl StyleResolver {
    /// Get the [`BackgroundColor`] from theme and state.
    pub fn background_color(&self) -> BackgroundColor {
        self.once.background_color.or(self.local.background_color).or(self.theme.background_color).unwrap_or_default()
    }

    /// Get the [`CircleColor`] from theme and state.
    pub fn circle_color(&self) -> CircleColor {
        self.once.circle_color.or(self.local.circle_color).or(self.theme.circle_color).unwrap_or_default()
    }

    /// Get the [`CircleWidth`] from theme and state.
    pub fn circle_width(&self) -> CircleWidth {
        self.once.circle_width.or(self.local.circle_width).or(self.theme.circle_width).unwrap_or_default()
    }

    /// Get the [`DiskEdgeColor`] from theme and state.
    pub fn disk_edge_color(&self) -> DiskEdgeColor {
        self.once.disk_edge_color.or(self.local.disk_edge_color).or(self.theme.disk_edge_color).unwrap_or_default()
    }

    /// Get the [`DiskEdgeWidth`] from theme and state.
    pub fn disk_edge_width(&self) -> DiskEdgeWidth {
        self.once.disk_edge_width.or(self.local.disk_edge_width).or(self.theme.disk_edge_width).unwrap_or_default()
    }

    /// Get the [`DiskFillColor`] from theme and state.
    pub fn disk_fill_color(&self) -> DiskFillColor {
        self.once.disk_fill_color.or(self.local.disk_fill_color).or(self.theme.disk_fill_color).unwrap_or_default()
    }

    /// Get the [`LineColor`] from theme and state.
    pub fn line_color(&self) -> LineColor {
        self.once.line_color.or(self.local.line_color).or(self.theme.line_color).unwrap_or_default()
    }

    /// Get the [`LineWidth`] from theme and state.
    pub fn line_width(&self) -> LineWidth {
        self.once.line_width.or(self.local.line_width).or(self.theme.line_width).unwrap_or_default()
    }

    /// Get the [`PointColor`] from theme and state.
    pub fn point_color(&self) -> PointColor {
        self.once.point_color.or(self.local.point_color).or(self.theme.point_color).unwrap_or_default()
    }

    /// Get the [`PointSize`] from theme and state.
    pub fn point_size(&self) -> PointSize {
        self.once.point_size.or(self.local.point_size).or(self.theme.point_size).unwrap_or_default()
    }

    /// Get the [`PolygonEdgeColor`] from theme and state.
    pub fn polygon_edge_color(&self) -> PolygonEdgeColor {
        self.once.polygon_edge_color.or(self.local.polygon_edge_color).or(self.theme.polygon_edge_color).unwrap_or_default()
    }

    /// Get the [`PolygonEdgeWidth`] from theme and state.
    pub fn polygon_edge_width(&self) -> PolygonEdgeWidth {
        self.once.polygon_edge_width.or(self.local.polygon_edge_width).or(self.theme.polygon_edge_width).unwrap_or_default()
    }

    /// Get the [`PolygonFillColor`] from theme and state.
    pub fn polygon_fill_color(&self) -> PolygonFillColor {
        self.once.polygon_fill_color.or(self.local.polygon_fill_color).or(self.theme.polygon_fill_color).unwrap_or_default()
    }

    /// Get the [`PolylineColor`] from theme and state.
    pub fn polyline_color(&self) -> PolylineColor {
        self.once.polyline_color.or(self.local.polyline_color).or(self.theme.polyline_color).unwrap_or_default()
    }

    /// Get the [`PolylineWidth`] from theme and state.
    pub fn polyline_width(&self) -> PolylineWidth {
        self.once.polyline_width.or(self.local.polyline_width).or(self.theme.polyline_width).unwrap_or_default()
    }

    /// Get the [`RectangleEdgeColor`] from theme and state.
    pub fn rectangle_edge_color(&self) -> RectangleEdgeColor {
        self.once
            .rectangle_edge_color
            .or(self.local.rectangle_edge_color)
            .or(self.theme.rectangle_edge_color)
            .unwrap_or_default()
    }

    /// Get the [`RectangleEdgeWidth`] from theme and state.
    pub fn rectangle_edge_width(&self) -> RectangleEdgeWidth {
        self.once
            .rectangle_edge_width
            .or(self.local.rectangle_edge_width)
            .or(self.theme.rectangle_edge_width)
            .unwrap_or_default()
    }

    /// Get the [`RectangleFillColor`] from theme and state.
    pub fn rectangle_fill_color(&self) -> RectangleFillColor {
        self.once
            .rectangle_fill_color
            .or(self.local.rectangle_fill_color)
            .or(self.theme.rectangle_fill_color)
            .unwrap_or_default()
    }

    /// Get the [`SquareEdgeColor`] from theme and state.
    pub fn square_edge_color(&self) -> SquareEdgeColor {
        self.once.square_edge_color.or(self.local.square_edge_color).or(self.theme.square_edge_color).unwrap_or_default()
    }

    /// Get the [`SquareEdgeWidth`] from theme and state.
    pub fn square_edge_width(&self) -> SquareEdgeWidth {
        self.once.square_edge_width.or(self.local.square_edge_width).or(self.theme.square_edge_width).unwrap_or_default()
    }

    /// Get the [`SquareFillColor`] from theme and state.
    pub fn square_fill_color(&self) -> SquareFillColor {
        self.once.square_fill_color.or(self.local.square_fill_color).or(self.theme.square_fill_color).unwrap_or_default()
    }

    /// Get the [`TextColor`] from theme and state.
    pub fn text_color(&self) -> TextColor {
        self.once.text_color.or(self.local.text_color).or(self.theme.text_color).unwrap_or_default()
    }

    /// Get the [`TextFont`] from theme and state.
    pub fn text_font(&self) -> TextFont {
        self.once.text_font.or(self.local.text_font).or(self.theme.text_font).unwrap_or_default()
    }

    /// Get the [`TextSize`] from theme and state.
    pub fn text_size(&self) -> TextSize {
        self.once.text_size.or(self.local.text_size).or(self.theme.text_size).unwrap_or_default()
    }

    /// Get the [`TriangleEdgeColor`] from theme and state.
    pub fn triangle_edge_color(&self) -> TriangleEdgeColor {
        self.once.triangle_edge_color.or(self.local.triangle_edge_color).or(self.theme.triangle_edge_color).unwrap_or_default()
    }

    /// Get the [`TriangleEdgeWidth`] from theme and state.
    pub fn triangle_edge_width(&self) -> TriangleEdgeWidth {
        self.once.triangle_edge_width.or(self.local.triangle_edge_width).or(self.theme.triangle_edge_width).unwrap_or_default()
    }

    /// Get the [`TriangleFillColor`] from theme and state.
    pub fn triangle_fill_color(&self) -> TriangleFillColor {
        self.once.triangle_fill_color.or(self.local.triangle_fill_color).or(self.theme.triangle_fill_color).unwrap_or_default()
    }
}
