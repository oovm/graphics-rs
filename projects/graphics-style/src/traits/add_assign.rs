use super::*;

impl AddAssign<PointSize> for PointStyle {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs.value);
    }
}

impl AddAssign<&PointSize> for PointStyle {
    fn add_assign(&mut self, rhs: &PointSize) {
        self.point_size = Some(rhs.value.clone());
    }
}

impl AddAssign<PointSize> for StyleContext {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs.value);
    }
}

impl AddAssign<&PointSize> for StyleContext {
    fn add_assign(&mut self, rhs: &PointSize) {
        self.point_size = Some(rhs.value);
    }
}
impl AddAssign<PointColor> for PointStyle {
    fn add_assign(&mut self, rhs: PointColor) {
        self.point_color = Some(rhs.value);
    }
}

impl AddAssign<&PointColor> for PointStyle {
    fn add_assign(&mut self, rhs: &PointColor) {
        self.point_color = Some(rhs.value.clone());
    }
}

impl AddAssign<PointColor> for StyleContext {
    fn add_assign(&mut self, rhs: PointColor) {
        self.point_color = Some(rhs.value);
    }
}

impl AddAssign<&PointColor> for StyleContext {
    fn add_assign(&mut self, rhs: &PointColor) {
        self.point_color = Some(rhs.value);
    }
}

impl AddAssign<LineWidth> for LineStyle {
    fn add_assign(&mut self, rhs: LineWidth) {
        self.line_width = Some(rhs.value);
    }
}

impl AddAssign<&LineWidth> for LineStyle {
    fn add_assign(&mut self, rhs: &LineWidth) {
        self.line_width = Some(rhs.value.clone());
    }
}

impl AddAssign<LineWidth> for StyleContext {
    fn add_assign(&mut self, rhs: LineWidth) {
        self.line_width = Some(rhs.value);
    }
}

impl AddAssign<&LineWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &LineWidth) {
        self.line_width = Some(rhs.value);
    }
}
impl AddAssign<LineColor> for LineStyle {
    fn add_assign(&mut self, rhs: LineColor) {
        self.line_color = Some(rhs.value);
    }
}

impl AddAssign<&LineColor> for LineStyle {
    fn add_assign(&mut self, rhs: &LineColor) {
        self.line_color = Some(rhs.value.clone());
    }
}

impl AddAssign<LineColor> for StyleContext {
    fn add_assign(&mut self, rhs: LineColor) {
        self.line_color = Some(rhs.value);
    }
}

impl AddAssign<&LineColor> for StyleContext {
    fn add_assign(&mut self, rhs: &LineColor) {
        self.line_color = Some(rhs.value);
    }
}

impl AddAssign<Self> for PointStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.point_size = rhs.point_size;
        self.point_color = rhs.point_color;
    }
}

impl AddAssign<&Self> for PointStyle {
    fn add_assign(&mut self, rhs: &Self) {
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
    fn add_assign(&mut self, rhs: &Self) {
        self.line_width = rhs.line_width.clone();
        self.line_color = rhs.line_color.clone();
    }
}
