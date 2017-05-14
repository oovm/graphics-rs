use super::*;

#[derive(Debug, Clone)]
pub(crate) struct StyleContext {
    pub point_size: Option<PointSize>,
    pub point_color: Option<PointColor>,
    pub line_color: Option<LineColor>,
    pub line_width: Option<LineWidth>,
}

impl StyleResolver {
    /// Set the value of [`PointSize`]
    pub fn point_size(&self) -> PointSize {
        self.local.point_size.unwrap_or(self.theme.point_size.unwrap_or(PointSize::default()))
    }
    /// Set the value of [`PointColor`]
    pub fn point_color(&self) -> PointColor {
        self.local.point_color.unwrap_or(self.theme.point_color.unwrap_or(PointColor::default()))
    }
    /// Set the value of [`LineColor`]
    pub fn line_color(&self) -> LineColor {
        self.local.line_color.unwrap_or(self.theme.line_color.unwrap_or(LineColor::default()))
    }
    /// Set the value of [`LineWidth`]
    pub fn line_width(&self) -> LineWidth {
        self.local.line_width.unwrap_or(self.theme.line_width.unwrap_or(LineWidth::default()))
    }
}

impl GraphicsStyle for PointSize {
    fn set_local_style(&self, context: &mut StyleResolver) {
        context.local.point_size = Some(self.clone());
    }
}

impl GraphicsStyle for PointColor {
    fn set_local_style(&self, context: &mut StyleResolver) {
        context.local.point_color = Some(self.clone());
    }
}

impl GraphicsStyle for LineColor {
    fn set_local_style(&self, context: &mut StyleResolver) {
        context.local.line_color = Some(self.clone());
    }
}

impl GraphicsStyle for LineWidth {
    fn set_local_style(&self, context: &mut StyleResolver) {
        context.local.line_width = Some(self.clone());
    }
}