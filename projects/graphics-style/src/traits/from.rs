use super::*;

impl From<PointSize> for GraphicsStyle {
    fn from(s: PointSize) -> Self {
        Self::PointSize(s.value)
    }
}

impl From<PointStyle> for GraphicsStyle {
    fn from(s: PointStyle) -> Self {
        Self::PointStyle(s.value)
    }
}
impl From<PointColor> for GraphicsStyle {
    fn from(s: PointColor) -> Self {
        Self::PointColor(s.value)
    }
}

impl From<PointStyle> for GraphicsStyle {
    fn from(s: PointStyle) -> Self {
        Self::PointStyle(s.value)
    }
}

impl From<LineWidth> for GraphicsStyle {
    fn from(s: LineWidth) -> Self {
        Self::LineWidth(s.value)
    }
}

impl From<LineStyle> for GraphicsStyle {
    fn from(s: LineStyle) -> Self {
        Self::LineStyle(s.value)
    }
}
impl From<LineColor> for GraphicsStyle {
    fn from(s: LineColor) -> Self {
        Self::LineColor(s.value)
    }
}

impl From<LineStyle> for GraphicsStyle {
    fn from(s: LineStyle) -> Self {
        Self::LineStyle(s.value)
    }
}
