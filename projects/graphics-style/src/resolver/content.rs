use super::*;

/// Get default style when not specified.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StyleContext {
    /// Get default [`PointSize`] when missing.
    pub point_size: Option<f32>,

    /// Get default [`PointColor`] when missing.
    pub point_color: Option<RGBA>,

    /// Get default [`CircleWidth`] when missing.
    pub circle_width: Option<f32>,

    /// Get default [`CircleColor`] when missing.
    pub circle_color: Option<RGBA>,

    /// Get default [`DiskFillColor`] when missing.
    pub disk_fill_color: Option<RGBA>,

    /// Get default [`DiskEdgeWidth`] when missing.
    pub disk_edge_width: Option<f32>,

    /// Get default [`DiskEdgeColor`] when missing.
    pub disk_edge_color: Option<RGBA>,

    /// Get default [`LineWidth`] when missing.
    pub line_width: Option<f32>,

    /// Get default [`LineColor`] when missing.
    pub line_color: Option<RGBA>,
}

impl StyleResolver {
    /// Set the value of [`PointSize`]
    pub fn point_size(&self) -> f32 {
        self.local.point_size.unwrap_or(self.theme.point_size.unwrap_or(PointSize::default().value).clone())
    }

    /// Set the value of [`PointColor`]
    pub fn point_color(&self) -> RGBA {
        self.local.point_color.unwrap_or(self.theme.point_color.unwrap_or(PointColor::default().value).clone())
    }

    /// Set the value of [`CircleWidth`]
    pub fn circle_width(&self) -> f32 {
        self.local.circle_width.unwrap_or(self.theme.circle_width.unwrap_or(CircleWidth::default().value).clone())
    }

    /// Set the value of [`CircleColor`]
    pub fn circle_color(&self) -> RGBA {
        self.local.circle_color.unwrap_or(self.theme.circle_color.unwrap_or(CircleColor::default().value).clone())
    }

    /// Set the value of [`DiskFillColor`]
    pub fn disk_fill_color(&self) -> RGBA {
        self.local.disk_fill_color.unwrap_or(self.theme.disk_fill_color.unwrap_or(DiskFillColor::default().value).clone())
    }

    /// Set the value of [`DiskEdgeWidth`]
    pub fn disk_edge_width(&self) -> f32 {
        self.local.disk_edge_width.unwrap_or(self.theme.disk_edge_width.unwrap_or(DiskEdgeWidth::default().value).clone())
    }

    /// Set the value of [`DiskEdgeColor`]
    pub fn disk_edge_color(&self) -> RGBA {
        self.local.disk_edge_color.unwrap_or(self.theme.disk_edge_color.unwrap_or(DiskEdgeColor::default().value).clone())
    }

    /// Set the value of [`LineWidth`]
    pub fn line_width(&self) -> f32 {
        self.local.line_width.unwrap_or(self.theme.line_width.unwrap_or(LineWidth::default().value).clone())
    }

    /// Set the value of [`LineColor`]
    pub fn line_color(&self) -> RGBA {
        self.local.line_color.unwrap_or(self.theme.line_color.unwrap_or(LineColor::default().value).clone())
    }
}
