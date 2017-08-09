use crate::*;

#[derive(Debug, Clone, Default)]
pub struct StyleContext {
    pub point_size: Option<f32>,
    pub point_color: Option<RGBA>,
    pub line_color: Option<RGBA>,
    pub line_width: Option<f32>,
}

impl StyleResolver {
    /// Set the value of [`PointSize`]
    pub fn point_size(&self) -> f32 {
        self.local.point_size.unwrap_or(self.theme.point_size.unwrap_or(PointSize::default().value))
    }
    /// Set the value of [`PointColor`]
    pub fn point_color(&self) -> RGBA {
        self.local.point_color.unwrap_or(self.theme.point_color.unwrap_or(PointColor::default().value))
    }
    /// Set the value of [`LineColor`]
    pub fn line_color(&self) -> RGBA {
        self.local.line_color.unwrap_or(self.theme.line_color.unwrap_or(LineColor::default().value))
    }
    /// Set the value of [`LineWidth`]
    pub fn line_width(&self) -> f32 {
        self.local.line_width.unwrap_or(self.theme.line_width.unwrap_or(LineWidth::default().value))
    }
}