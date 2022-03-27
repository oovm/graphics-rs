use super::*;

/// Get default style when not specified.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StyleContext {
    /// Get default [`PointSize`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_size: Option<PointSize>,

    /// Get default [`PointColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub point_color: Option<PointColor>,

    /// Get default [`CircleWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_width: Option<CircleWidth>,

    /// Get default [`CircleColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_color: Option<CircleColor>,

    /// Get default [`DiskFillColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_fill_color: Option<DiskFillColor>,

    /// Get default [`DiskEdgeWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_edge_width: Option<DiskEdgeWidth>,

    /// Get default [`DiskEdgeColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_edge_color: Option<DiskEdgeColor>,

    /// Get default [`LineWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// Get default [`LineColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_color: Option<LineColor>,

    /// Get default [`TriangleEdgeWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_edge_width: Option<TriangleEdgeWidth>,

    /// Get default [`TriangleFillColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triangle_fill_color: Option<TriangleFillColor>,

    /// Get default [`SquareEdgeWidth`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_edge_width: Option<SquareEdgeWidth>,

    /// Get default [`SquareFillColor`] when missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_fill_color: Option<SquareFillColor>,
}

impl StyleResolver {
    /// Get the [`PointStyle`] from theme and state.
    pub fn point_style(&self) -> PointStyle {
        PointStyle { point_size: Some(self.point_size()), point_color: Some(self.point_color()) }
    }

    /// Get the [`Point3DStyle`] from theme and state.
    pub fn point3D_style(&self) -> Point3DStyle {
        Point3DStyle { point_size: Some(self.point_size()), point_color: Some(self.point_color()) }
    }

    /// Get the [`CircleStyle`] from theme and state.
    pub fn circle_style(&self) -> CircleStyle {
        CircleStyle { circle_width: Some(self.circle_width()), circle_color: Some(self.circle_color()) }
    }

    /// Get the [`DiskStyle`] from theme and state.
    pub fn disk_style(&self) -> DiskStyle {
        DiskStyle {
            disk_fill_color: Some(self.disk_fill_color()),
            disk_edge_width: Some(self.disk_edge_width()),
            disk_edge_color: Some(self.disk_edge_color()),
        }
    }

    /// Get the [`LineStyle`] from theme and state.
    pub fn line_style(&self) -> LineStyle {
        LineStyle { line_width: Some(self.line_width()), line_color: Some(self.line_color()) }
    }

    /// Get the [`TriangleStyle`] from theme and state.
    pub fn triangle_style(&self) -> TriangleStyle {
        TriangleStyle {
            triangle_edge_width: Some(self.triangle_edge_width()),
            triangle_fill_color: Some(self.triangle_fill_color()),
        }
    }

    /// Get the [`SquareStyle`] from theme and state.
    pub fn square_style(&self) -> SquareStyle {
        SquareStyle { square_edge_width: Some(self.square_edge_width()), square_fill_color: Some(self.square_fill_color()) }
    }

    /// Get the [`RectangleStyle`] from theme and state.
    pub fn rectangle_style(&self) -> RectangleStyle {
        RectangleStyle { square_edge_width: Some(self.square_edge_width()), square_fill_color: Some(self.square_fill_color()) }
    }

    /// Get the [`PointSize`] from theme and state.
    pub fn point_size(&self) -> PointSize {
        self.once.point_size.or(self.local.point_size).or(self.theme.point_size).unwrap_or_default()
    }

    /// Get the [`PointColor`] from theme and state.
    pub fn point_color(&self) -> PointColor {
        self.once.point_color.or(self.local.point_color).or(self.theme.point_color).unwrap_or_default()
    }

    /// Get the [`CircleWidth`] from theme and state.
    pub fn circle_width(&self) -> CircleWidth {
        self.once.circle_width.or(self.local.circle_width).or(self.theme.circle_width).unwrap_or_default()
    }

    /// Get the [`CircleColor`] from theme and state.
    pub fn circle_color(&self) -> CircleColor {
        self.once.circle_color.or(self.local.circle_color).or(self.theme.circle_color).unwrap_or_default()
    }

    /// Get the [`DiskFillColor`] from theme and state.
    pub fn disk_fill_color(&self) -> DiskFillColor {
        self.once.disk_fill_color.or(self.local.disk_fill_color).or(self.theme.disk_fill_color).unwrap_or_default()
    }

    /// Get the [`DiskEdgeWidth`] from theme and state.
    pub fn disk_edge_width(&self) -> DiskEdgeWidth {
        self.once.disk_edge_width.or(self.local.disk_edge_width).or(self.theme.disk_edge_width).unwrap_or_default()
    }

    /// Get the [`DiskEdgeColor`] from theme and state.
    pub fn disk_edge_color(&self) -> DiskEdgeColor {
        self.once.disk_edge_color.or(self.local.disk_edge_color).or(self.theme.disk_edge_color).unwrap_or_default()
    }

    /// Get the [`LineWidth`] from theme and state.
    pub fn line_width(&self) -> LineWidth {
        self.once.line_width.or(self.local.line_width).or(self.theme.line_width).unwrap_or_default()
    }

    /// Get the [`LineColor`] from theme and state.
    pub fn line_color(&self) -> LineColor {
        self.once.line_color.or(self.local.line_color).or(self.theme.line_color).unwrap_or_default()
    }

    /// Get the [`TriangleEdgeWidth`] from theme and state.
    pub fn triangle_edge_width(&self) -> TriangleEdgeWidth {
        self.once.triangle_edge_width.or(self.local.triangle_edge_width).or(self.theme.triangle_edge_width).unwrap_or_default()
    }

    /// Get the [`TriangleFillColor`] from theme and state.
    pub fn triangle_fill_color(&self) -> TriangleFillColor {
        self.once.triangle_fill_color.or(self.local.triangle_fill_color).or(self.theme.triangle_fill_color).unwrap_or_default()
    }

    /// Get the [`SquareEdgeWidth`] from theme and state.
    pub fn square_edge_width(&self) -> SquareEdgeWidth {
        self.once.square_edge_width.or(self.local.square_edge_width).or(self.theme.square_edge_width).unwrap_or_default()
    }

    /// Get the [`SquareFillColor`] from theme and state.
    pub fn square_fill_color(&self) -> SquareFillColor {
        self.once.square_fill_color.or(self.local.square_fill_color).or(self.theme.square_fill_color).unwrap_or_default()
    }
}
