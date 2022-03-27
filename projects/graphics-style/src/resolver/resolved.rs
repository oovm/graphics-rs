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
