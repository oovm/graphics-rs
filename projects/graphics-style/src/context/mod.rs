use crate::*;

/// Resolve missing style
#[derive(Debug, Clone)]
pub struct StyleResolver {
    theme: StyleContext,
    local: StyleContext,
}

/// All available styles.
#[derive(Debug, Clone)]
pub enum GraphicsStyle {
    /// Set the missing style of PointSize
    PointSize(PointSize),
    /// Set the missing style of PointColor
    PointColor(RGBA),
    /// Set the missing style of LineColor
    LineColor(RGBA),
    /// Set the missing style of LineWidth
    LineWidth(LineWidth),
}

#[derive(Debug, Clone)]
pub(crate) struct StyleContext {
    pub point_size: Option<PointSize>,
    pub point_color: Option<RGBA>,
    pub line_color: Option<RGBA>,
    pub line_width: Option<LineWidth>,
}

impl StyleResolver {
    /// Set the value of [`PointSize`]
    pub fn point_size(&self) -> PointSize {
        self.local.point_size.unwrap_or(self.theme.point_size.unwrap_or(PointSize::default()))
    }
    /// Set the value of [`RGBA`]
    pub fn point_color(&self) -> RGBA {
        self.local.point_color.unwrap_or(self.theme.point_color.unwrap_or(RGBA::default()))
    }
    /// Set the value of [`RGBA`]
    pub fn line_color(&self) -> RGBA {
        self.local.line_color.unwrap_or(self.theme.line_color.unwrap_or(RGBA::default()))
    }
    /// Set the value of [`LineWidth`]
    pub fn line_width(&self) -> LineWidth {
        self.local.line_width.unwrap_or(self.theme.line_width.unwrap_or(LineWidth::default()))
    }
}

impl StyleResolver {
    /// Set the style of the given element.
    pub fn set_local_style<T>(&mut self, style: T)
    where
        T: Into<GraphicsStyle>,
    {
        match style.into() {
            GraphicsStyle::PointSize(s) => self.local.point_size = Some(s.clone()),
            GraphicsStyle::PointColor(s) => self.local.point_color = Some(s.clone()),
            GraphicsStyle::LineColor(s) => self.local.line_color = Some(s.clone()),
            GraphicsStyle::LineWidth(s) => self.local.line_width = Some(s.clone()),
        }
    }
}

impl From<PointSize> for GraphicsStyle {
    fn from(s: PointSize) -> Self {
        Self::PointSize(s)
    }
}

impl From<LineWidth> for GraphicsStyle {
    fn from(s: LineWidth) -> Self {
        Self::LineWidth(s)
    }
}
