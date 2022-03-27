use super::*;

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct BackgroundStyle {
    /// Get the config of [`crate::BackgroundColor`]
    pub background_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct CircleStyle {
    /// Get the config of [`crate::CircleWidth`]
    pub circle_width: f32,

    /// Get the config of [`crate::CircleColor`]
    pub circle_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskStyle {
    /// Get the config of [`crate::DiskFillColor`]
    pub disk_fill_color: RGBA,

    /// Get the config of [`crate::DiskEdgeWidth`]
    pub disk_edge_width: f32,

    /// Get the config of [`crate::DiskEdgeColor`]
    pub disk_edge_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeColor {
    /// Get the config of [`crate::DiskEdgeColor`]
    pub disk_edge_color: RGBA,

    /// Get the config of [`crate::TriangleEdgeColor`]
    pub triangle_edge_color: RGBA,

    /// Get the config of [`crate::SquareEdgeColor`]
    pub square_edge_color: RGBA,

    /// Get the config of [`crate::RectangleEdgeColor`]
    pub rectangle_edge_color: RGBA,

    /// Get the config of [`crate::PolygonEdgeColor`]
    pub polygon_edge_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeStyle {
    /// Get the config of [`crate::DiskEdgeWidth`]
    pub disk_edge_width: f32,

    /// Get the config of [`crate::TriangleEdgeColor`]
    pub triangle_edge_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct EdgeWidth {
    /// Get the config of [`crate::DiskEdgeWidth`]
    pub disk_edge_width: f32,

    /// Get the config of [`crate::TriangleEdgeWidth`]
    pub triangle_edge_width: f32,

    /// Get the config of [`crate::SquareEdgeWidth`]
    pub square_edge_width: f32,

    /// Get the config of [`crate::RectangleEdgeWidth`]
    pub rectangle_edge_width: f32,

    /// Get the config of [`crate::PolygonEdgeWidth`]
    pub polygon_edge_width: f32,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FillColor {
    /// Get the config of [`crate::DiskFillColor`]
    pub disk_fill_color: RGBA,

    /// Get the config of [`crate::TriangleFillColor`]
    pub triangle_fill_color: RGBA,

    /// Get the config of [`crate::SquareFillColor`]
    pub square_fill_color: RGBA,

    /// Get the config of [`crate::RectangleFillColor`]
    pub rectangle_fill_color: RGBA,

    /// Get the config of [`crate::PolygonFillColor`]
    pub polygon_fill_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct LineStyle {
    /// Get the config of [`crate::LineWidth`]
    pub line_width: f32,

    /// Get the config of [`crate::LineColor`]
    pub line_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PointStyle {
    /// Get the config of [`crate::PointSize`]
    pub point_size: f32,

    /// Get the config of [`crate::PointColor`]
    pub point_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolygonStyle {
    /// Get the config of [`crate::PolygonFillColor`]
    pub polygon_fill_color: RGBA,

    /// Get the config of [`crate::PolygonEdgeWidth`]
    pub polygon_edge_width: f32,

    /// Get the config of [`crate::PolygonEdgeColor`]
    pub polygon_edge_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct PolylineStyle {
    /// Get the config of [`crate::PolylineWidth`]
    pub polyline_width: f32,

    /// Get the config of [`crate::PolylineColor`]
    pub polyline_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct RectangleStyle {
    /// Get the config of [`crate::RectangleFillColor`]
    pub rectangle_fill_color: RGBA,

    /// Get the config of [`crate::RectangleEdgeWidth`]
    pub rectangle_edge_width: f32,

    /// Get the config of [`crate::RectangleEdgeColor`]
    pub rectangle_edge_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SquareStyle {
    /// Get the config of [`crate::SquareFillColor`]
    pub square_fill_color: RGBA,

    /// Get the config of [`crate::SquareEdgeWidth`]
    pub square_edge_width: f32,

    /// Get the config of [`crate::SquareEdgeColor`]
    pub square_edge_color: RGBA,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TextStyle {
    /// Get the config of [`crate::TextColor`]
    pub text_color: RGBA,

    /// Get the config of [`crate::TextSize`]
    pub text_size: f32,

    /// Get the config of [`crate::TextFont`]
    pub text_font: f32,
}

/// Get default style when not specified.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct TriangleStyle {
    /// Get the config of [`crate::TriangleFillColor`]
    pub triangle_fill_color: RGBA,

    /// Get the config of [`crate::TriangleEdgeWidth`]
    pub triangle_edge_width: f32,

    /// Get the config of [`crate::TriangleEdgeColor`]
    pub triangle_edge_color: RGBA,
}

impl StyleResolver {
    /// Get the [`crate::BackgroundStyle`] from theme and state.
    pub fn resolve_background_style(&self, style: crate::BackgroundStyle) -> BackgroundStyle {
        BackgroundStyle { background_color: style.background_color.unwrap_or(self.background_color()).value }
    }

    /// Get the [`crate::CircleStyle`] from theme and state.
    pub fn resolve_circle_style(&self, style: crate::CircleStyle) -> CircleStyle {
        CircleStyle {
            circle_width: style.circle_width.unwrap_or(self.circle_width()).value,
            circle_color: style.circle_color.unwrap_or(self.circle_color()).value,
        }
    }

    /// Get the [`crate::DiskStyle`] from theme and state.
    pub fn resolve_disk_style(&self, style: crate::DiskStyle) -> DiskStyle {
        DiskStyle {
            disk_fill_color: style.disk_fill_color.unwrap_or(self.disk_fill_color()).value,
            disk_edge_width: style.disk_edge_width.unwrap_or(self.disk_edge_width()).value,
            disk_edge_color: style.disk_edge_color.unwrap_or(self.disk_edge_color()).value,
        }
    }

    /// Get the [`crate::EdgeColor`] from theme and state.
    pub fn resolve_edge_color(&self, style: crate::EdgeColor) -> EdgeColor {
        EdgeColor {
            disk_edge_color: style.disk_edge_color.unwrap_or(self.disk_edge_color()).value,
            triangle_edge_color: style.triangle_edge_color.unwrap_or(self.triangle_edge_color()).value,
            square_edge_color: style.square_edge_color.unwrap_or(self.square_edge_color()).value,
            rectangle_edge_color: style.rectangle_edge_color.unwrap_or(self.rectangle_edge_color()).value,
            polygon_edge_color: style.polygon_edge_color.unwrap_or(self.polygon_edge_color()).value,
        }
    }

    /// Get the [`crate::EdgeStyle`] from theme and state.
    pub fn resolve_edge_style(&self, style: crate::EdgeStyle) -> EdgeStyle {
        EdgeStyle {
            disk_edge_width: style.disk_edge_width.unwrap_or(self.disk_edge_width()).value,
            triangle_edge_color: style.triangle_edge_color.unwrap_or(self.triangle_edge_color()).value,
        }
    }

    /// Get the [`crate::EdgeWidth`] from theme and state.
    pub fn resolve_edge_width(&self, style: crate::EdgeWidth) -> EdgeWidth {
        EdgeWidth {
            disk_edge_width: style.disk_edge_width.unwrap_or(self.disk_edge_width()).value,
            triangle_edge_width: style.triangle_edge_width.unwrap_or(self.triangle_edge_width()).value,
            square_edge_width: style.square_edge_width.unwrap_or(self.square_edge_width()).value,
            rectangle_edge_width: style.rectangle_edge_width.unwrap_or(self.rectangle_edge_width()).value,
            polygon_edge_width: style.polygon_edge_width.unwrap_or(self.polygon_edge_width()).value,
        }
    }

    /// Get the [`crate::FillColor`] from theme and state.
    pub fn resolve_fill_color(&self, style: crate::FillColor) -> FillColor {
        FillColor {
            disk_fill_color: style.disk_fill_color.unwrap_or(self.disk_fill_color()).value,
            triangle_fill_color: style.triangle_fill_color.unwrap_or(self.triangle_fill_color()).value,
            square_fill_color: style.square_fill_color.unwrap_or(self.square_fill_color()).value,
            rectangle_fill_color: style.rectangle_fill_color.unwrap_or(self.rectangle_fill_color()).value,
            polygon_fill_color: style.polygon_fill_color.unwrap_or(self.polygon_fill_color()).value,
        }
    }

    /// Get the [`crate::LineStyle`] from theme and state.
    pub fn resolve_line_style(&self, style: crate::LineStyle) -> LineStyle {
        LineStyle {
            line_width: style.line_width.unwrap_or(self.line_width()).value,
            line_color: style.line_color.unwrap_or(self.line_color()).value,
        }
    }

    /// Get the [`crate::PointStyle`] from theme and state.
    pub fn resolve_point_style(&self, style: crate::PointStyle) -> PointStyle {
        PointStyle {
            point_size: style.point_size.unwrap_or(self.point_size()).value,
            point_color: style.point_color.unwrap_or(self.point_color()).value,
        }
    }

    /// Get the [`crate::PolygonStyle`] from theme and state.
    pub fn resolve_polygon_style(&self, style: crate::PolygonStyle) -> PolygonStyle {
        PolygonStyle {
            polygon_fill_color: style.polygon_fill_color.unwrap_or(self.polygon_fill_color()).value,
            polygon_edge_width: style.polygon_edge_width.unwrap_or(self.polygon_edge_width()).value,
            polygon_edge_color: style.polygon_edge_color.unwrap_or(self.polygon_edge_color()).value,
        }
    }

    /// Get the [`crate::PolylineStyle`] from theme and state.
    pub fn resolve_polyline_style(&self, style: crate::PolylineStyle) -> PolylineStyle {
        PolylineStyle {
            polyline_width: style.polyline_width.unwrap_or(self.polyline_width()).value,
            polyline_color: style.polyline_color.unwrap_or(self.polyline_color()).value,
        }
    }

    /// Get the [`crate::RectangleStyle`] from theme and state.
    pub fn resolve_rectangle_style(&self, style: crate::RectangleStyle) -> RectangleStyle {
        RectangleStyle {
            rectangle_fill_color: style.rectangle_fill_color.unwrap_or(self.rectangle_fill_color()).value,
            rectangle_edge_width: style.rectangle_edge_width.unwrap_or(self.rectangle_edge_width()).value,
            rectangle_edge_color: style.rectangle_edge_color.unwrap_or(self.rectangle_edge_color()).value,
        }
    }

    /// Get the [`crate::SquareStyle`] from theme and state.
    pub fn resolve_square_style(&self, style: crate::SquareStyle) -> SquareStyle {
        SquareStyle {
            square_fill_color: style.square_fill_color.unwrap_or(self.square_fill_color()).value,
            square_edge_width: style.square_edge_width.unwrap_or(self.square_edge_width()).value,
            square_edge_color: style.square_edge_color.unwrap_or(self.square_edge_color()).value,
        }
    }

    /// Get the [`crate::TextStyle`] from theme and state.
    pub fn resolve_text_style(&self, style: crate::TextStyle) -> TextStyle {
        TextStyle {
            text_color: style.text_color.unwrap_or(self.text_color()).value,
            text_size: style.text_size.unwrap_or(self.text_size()).value,
            text_font: style.text_font.unwrap_or(self.text_font()).value,
        }
    }

    /// Get the [`crate::TriangleStyle`] from theme and state.
    pub fn resolve_triangle_style(&self, style: crate::TriangleStyle) -> TriangleStyle {
        TriangleStyle {
            triangle_fill_color: style.triangle_fill_color.unwrap_or(self.triangle_fill_color()).value,
            triangle_edge_width: style.triangle_edge_width.unwrap_or(self.triangle_edge_width()).value,
            triangle_edge_color: style.triangle_edge_color.unwrap_or(self.triangle_edge_color()).value,
        }
    }
}
