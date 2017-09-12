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
impl AddAssign<DiskFillColor> for DiskStyle {
    fn add_assign(&mut self, rhs: DiskFillColor) {
        self.disk_fill_color = Some(rhs.value);
    }
}

impl AddAssign<&DiskFillColor> for DiskStyle {
    fn add_assign(&mut self, rhs: &DiskFillColor) {
        self.disk_fill_color = Some(rhs.value.clone());
    }
}

impl AddAssign<DiskFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: DiskFillColor) {
        self.disk_fill_color = Some(rhs.value);
    }
}

impl AddAssign<&DiskFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: &DiskFillColor) {
        self.disk_fill_color = Some(rhs.value);
    }
}
impl AddAssign<DiskEdgeWidth> for DiskStyle {
    fn add_assign(&mut self, rhs: DiskEdgeWidth) {
        self.disk_edge_width = Some(rhs.value);
    }
}

impl AddAssign<&DiskEdgeWidth> for DiskStyle {
    fn add_assign(&mut self, rhs: &DiskEdgeWidth) {
        self.disk_edge_width = Some(rhs.value.clone());
    }
}

impl AddAssign<DiskEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: DiskEdgeWidth) {
        self.disk_edge_width = Some(rhs.value);
    }
}

impl AddAssign<&DiskEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &DiskEdgeWidth) {
        self.disk_edge_width = Some(rhs.value);
    }
}
impl AddAssign<DiskEdgeColor> for DiskStyle {
    fn add_assign(&mut self, rhs: DiskEdgeColor) {
        self.disk_edge_color = Some(rhs.value);
    }
}

impl AddAssign<&DiskEdgeColor> for DiskStyle {
    fn add_assign(&mut self, rhs: &DiskEdgeColor) {
        self.disk_edge_color = Some(rhs.value.clone());
    }
}

impl AddAssign<DiskEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: DiskEdgeColor) {
        self.disk_edge_color = Some(rhs.value);
    }
}

impl AddAssign<&DiskEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: &DiskEdgeColor) {
        self.disk_edge_color = Some(rhs.value);
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
    fn add_assign(&mut self, rhs: Self) {self.point_size = rhs.point_size;self.point_color = rhs.point_color;}
}

impl AddAssign<&Self> for PointStyle {
    fn add_assign(&mut self, rhs: &Self) {self.point_size = rhs.point_size.clone();self.point_color = rhs.point_color.clone();}
}

impl AddAssign<Self> for CircleStyle {
    fn add_assign(&mut self, rhs: Self) {self.circle_width = rhs.circle_width;self.circle_color = rhs.circle_color;}
}

impl AddAssign<&Self> for CircleStyle {
    fn add_assign(&mut self, rhs: &Self) {self.circle_width = rhs.circle_width.clone();self.circle_color = rhs.circle_color.clone();}
}

impl AddAssign<Self> for DiskStyle {
    fn add_assign(&mut self, rhs: Self) {self.disk_fill_color = rhs.disk_fill_color;self.disk_edge_width = rhs.disk_edge_width;self.disk_edge_color = rhs.disk_edge_color;}
}

impl AddAssign<&Self> for DiskStyle {
    fn add_assign(&mut self, rhs: &Self) {self.disk_fill_color = rhs.disk_fill_color.clone();self.disk_edge_width = rhs.disk_edge_width.clone();self.disk_edge_color = rhs.disk_edge_color.clone();}
}

impl AddAssign<Self> for LineStyle {
    fn add_assign(&mut self, rhs: Self) {self.line_width = rhs.line_width;self.line_color = rhs.line_color;}
}

impl AddAssign<&Self> for LineStyle {
    fn add_assign(&mut self, rhs: &Self) {self.line_width = rhs.line_width.clone();self.line_color = rhs.line_color.clone();}
}