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

impl AddAssign<CircleWidth> for CircleStyle {
    fn add_assign(&mut self, rhs: CircleWidth) {
        self.circle_width = Some(rhs.value);
    }
}

impl AddAssign<&CircleWidth> for CircleStyle {
    fn add_assign(&mut self, rhs: &CircleWidth) {
        self.circle_width = Some(rhs.value.clone());
    }
}

impl AddAssign<CircleWidth> for StyleContext {
    fn add_assign(&mut self, rhs: CircleWidth) {
        self.circle_width = Some(rhs.value);
    }
}

impl AddAssign<&CircleWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &CircleWidth) {
        self.circle_width = Some(rhs.value);
    }
}

impl AddAssign<CircleColor> for CircleStyle {
    fn add_assign(&mut self, rhs: CircleColor) {
        self.circle_color = Some(rhs.value);
    }
}

impl AddAssign<&CircleColor> for CircleStyle {
    fn add_assign(&mut self, rhs: &CircleColor) {
        self.circle_color = Some(rhs.value.clone());
    }
}

impl AddAssign<CircleColor> for StyleContext {
    fn add_assign(&mut self, rhs: CircleColor) {
        self.circle_color = Some(rhs.value);
    }
}

impl AddAssign<&CircleColor> for StyleContext {
    fn add_assign(&mut self, rhs: &CircleColor) {
        self.circle_color = Some(rhs.value);
    }
}

impl AddAssign<FillColor> for DiskStyle {
    fn add_assign(&mut self, rhs: FillColor) {
        self.fill_color = Some(rhs.value);
    }
}

impl AddAssign<&FillColor> for DiskStyle {
    fn add_assign(&mut self, rhs: &FillColor) {
        self.fill_color = Some(rhs.value.clone());
    }
}

impl AddAssign<FillColor> for StyleContext {
    fn add_assign(&mut self, rhs: FillColor) {
        self.fill_color = Some(rhs.value);
    }
}

impl AddAssign<&FillColor> for StyleContext {
    fn add_assign(&mut self, rhs: &FillColor) {
        self.fill_color = Some(rhs.value);
    }
}

impl AddAssign<EdgeWidth> for DiskStyle {
    fn add_assign(&mut self, rhs: EdgeWidth) {
        self.edge_width = Some(rhs.value);
    }
}

impl AddAssign<&EdgeWidth> for DiskStyle {
    fn add_assign(&mut self, rhs: &EdgeWidth) {
        self.edge_width = Some(rhs.value.clone());
    }
}

impl AddAssign<EdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: EdgeWidth) {
        self.edge_width = Some(rhs.value);
    }
}

impl AddAssign<&EdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &EdgeWidth) {
        self.edge_width = Some(rhs.value);
    }
}

impl AddAssign<EdgeColor> for DiskStyle {
    fn add_assign(&mut self, rhs: EdgeColor) {
        self.edge_color = Some(rhs.value);
    }
}

impl AddAssign<&EdgeColor> for DiskStyle {
    fn add_assign(&mut self, rhs: &EdgeColor) {
        self.edge_color = Some(rhs.value.clone());
    }
}

impl AddAssign<EdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: EdgeColor) {
        self.edge_color = Some(rhs.value);
    }
}

impl AddAssign<&EdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: &EdgeColor) {
        self.edge_color = Some(rhs.value);
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

impl AddAssign<Self> for CircleStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.circle_width = rhs.circle_width;
        self.circle_color = rhs.circle_color;
    }
}

impl AddAssign<&Self> for CircleStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.circle_width = rhs.circle_width.clone();
        self.circle_color = rhs.circle_color.clone();
    }
}

impl AddAssign<Self> for DiskStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.fill_color = rhs.fill_color;
        self.edge_width = rhs.edge_width;
        self.edge_color = rhs.edge_color;
    }
}

impl AddAssign<&Self> for DiskStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.fill_color = rhs.fill_color.clone();
        self.edge_width = rhs.edge_width.clone();
        self.edge_color = rhs.edge_color.clone();
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
