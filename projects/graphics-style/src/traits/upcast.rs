use super::*;

impl From<PointSize> for GraphicsStyle {
    fn from(s: PointSize) -> Self {
        Self::PointSize(s.value)
    }
}

impl From<PointColor> for GraphicsStyle {
    fn from(s: PointColor) -> Self {
        Self::PointColor(s.value)
    }
}

impl From<LineWidth> for GraphicsStyle {
    fn from(s: LineWidth) -> Self {
        Self::LineWidth(s.value)
    }
}

impl From<LineColor> for GraphicsStyle {
    fn from(s: LineColor) -> Self {
        Self::LineColor(s.value)
    }
}

impl AddAssign<Self> for PointStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.point_size = rhs.point_size;
        self.point_color = rhs.point_color;
    }
}

impl AddAssign<&Self> for PointStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.point_size = rhs.point_size.clone();
        self.point_color = rhs.point_color.clone();
    }
}

impl AddAssign<Self> for LineStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.line_width = rhs.line_width;
        self.line_color = rhs.line_color;
    }
}

impl AddAssign<&Self> for LineStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.line_width = rhs.line_width.clone();
        self.line_color = rhs.line_color.clone();
    }
}

impl From<PointSize> for GraphicsStyle {
    fn from(s: PointSize) -> Self {
        Self::PointSize(s.value)
    }
}

impl From<PointColor> for GraphicsStyle {
    fn from(s: PointColor) -> Self {
        Self::PointColor(s.value)
    }
}

impl From<LineWidth> for GraphicsStyle {
    fn from(s: LineWidth) -> Self {
        Self::LineWidth(s.value)
    }
}

impl From<LineColor> for GraphicsStyle {
    fn from(s: LineColor) -> Self {
        Self::LineColor(s.value)
    }
}

impl From<PointSize> for GraphicsStyle {
    fn from(s: PointSize) -> Self {
        Self::PointSize(s.value)
    }
}

impl From<PointColor> for GraphicsStyle {
    fn from(s: PointColor) -> Self {
        Self::PointColor(s.value)
    }
}

impl From<LineWidth> for GraphicsStyle {
    fn from(s: LineWidth) -> Self {
        Self::LineWidth(s.value)
    }
}

impl From<LineColor> for GraphicsStyle {
    fn from(s: LineColor) -> Self {
        Self::LineColor(s.value)
    }
}
