use super::*;

impl AddAssign<PointSize> for PointStyle {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs);
    }
}

impl AddAssign<&PointSize> for PointStyle {
    fn add_assign(&mut self, rhs: &PointSize) {
        self.point_size = Some(rhs.clone());
    }
}

impl AddAssign<PointSize> for StyleContext {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs);
    }
}

impl AddAssign<&PointSize> for StyleContext {
    fn add_assign(&mut self, rhs: &PointSize) {
        self.point_size = Some(rhs.clone());
    }
}

impl AddAssign<PointColor> for PointStyle {
    fn add_assign(&mut self, rhs: PointColor) {
        self.point_color = Some(rhs);
    }
}

impl AddAssign<&PointColor> for PointStyle {
    fn add_assign(&mut self, rhs: &PointColor) {
        self.point_color = Some(rhs.clone());
    }
}

impl AddAssign<PointColor> for StyleContext {
    fn add_assign(&mut self, rhs: PointColor) {
        self.point_color = Some(rhs);
    }
}

impl AddAssign<&PointColor> for StyleContext {
    fn add_assign(&mut self, rhs: &PointColor) {
        self.point_color = Some(rhs.clone());
    }
}

impl AddAssign<CircleWidth> for CircleStyle {
    fn add_assign(&mut self, rhs: CircleWidth) {
        self.circle_width = Some(rhs);
    }
}

impl AddAssign<&CircleWidth> for CircleStyle {
    fn add_assign(&mut self, rhs: &CircleWidth) {
        self.circle_width = Some(rhs.clone());
    }
}

impl AddAssign<CircleWidth> for StyleContext {
    fn add_assign(&mut self, rhs: CircleWidth) {
        self.circle_width = Some(rhs);
    }
}

impl AddAssign<&CircleWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &CircleWidth) {
        self.circle_width = Some(rhs.clone());
    }
}

impl AddAssign<CircleColor> for CircleStyle {
    fn add_assign(&mut self, rhs: CircleColor) {
        self.circle_color = Some(rhs);
    }
}

impl AddAssign<&CircleColor> for CircleStyle {
    fn add_assign(&mut self, rhs: &CircleColor) {
        self.circle_color = Some(rhs.clone());
    }
}

impl AddAssign<CircleColor> for StyleContext {
    fn add_assign(&mut self, rhs: CircleColor) {
        self.circle_color = Some(rhs);
    }
}

impl AddAssign<&CircleColor> for StyleContext {
    fn add_assign(&mut self, rhs: &CircleColor) {
        self.circle_color = Some(rhs.clone());
    }
}

impl AddAssign<DiskFillColor> for DiskStyle {
    fn add_assign(&mut self, rhs: DiskFillColor) {
        self.disk_fill_color = Some(rhs);
    }
}

impl AddAssign<&DiskFillColor> for DiskStyle {
    fn add_assign(&mut self, rhs: &DiskFillColor) {
        self.disk_fill_color = Some(rhs.clone());
    }
}

impl AddAssign<DiskFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: DiskFillColor) {
        self.disk_fill_color = Some(rhs);
    }
}

impl AddAssign<&DiskFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: &DiskFillColor) {
        self.disk_fill_color = Some(rhs.clone());
    }
}

impl AddAssign<DiskEdgeWidth> for DiskStyle {
    fn add_assign(&mut self, rhs: DiskEdgeWidth) {
        self.disk_edge_width = Some(rhs);
    }
}

impl AddAssign<&DiskEdgeWidth> for DiskStyle {
    fn add_assign(&mut self, rhs: &DiskEdgeWidth) {
        self.disk_edge_width = Some(rhs.clone());
    }
}

impl AddAssign<DiskEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: DiskEdgeWidth) {
        self.disk_edge_width = Some(rhs);
    }
}

impl AddAssign<&DiskEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &DiskEdgeWidth) {
        self.disk_edge_width = Some(rhs.clone());
    }
}

impl AddAssign<DiskEdgeColor> for DiskStyle {
    fn add_assign(&mut self, rhs: DiskEdgeColor) {
        self.disk_edge_color = Some(rhs);
    }
}

impl AddAssign<&DiskEdgeColor> for DiskStyle {
    fn add_assign(&mut self, rhs: &DiskEdgeColor) {
        self.disk_edge_color = Some(rhs.clone());
    }
}

impl AddAssign<DiskEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: DiskEdgeColor) {
        self.disk_edge_color = Some(rhs);
    }
}

impl AddAssign<&DiskEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: &DiskEdgeColor) {
        self.disk_edge_color = Some(rhs.clone());
    }
}

impl AddAssign<LineWidth> for LineStyle {
    fn add_assign(&mut self, rhs: LineWidth) {
        self.line_width = Some(rhs);
    }
}

impl AddAssign<&LineWidth> for LineStyle {
    fn add_assign(&mut self, rhs: &LineWidth) {
        self.line_width = Some(rhs.clone());
    }
}

impl AddAssign<LineWidth> for StyleContext {
    fn add_assign(&mut self, rhs: LineWidth) {
        self.line_width = Some(rhs);
    }
}

impl AddAssign<&LineWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &LineWidth) {
        self.line_width = Some(rhs.clone());
    }
}

impl AddAssign<LineColor> for LineStyle {
    fn add_assign(&mut self, rhs: LineColor) {
        self.line_color = Some(rhs);
    }
}

impl AddAssign<&LineColor> for LineStyle {
    fn add_assign(&mut self, rhs: &LineColor) {
        self.line_color = Some(rhs.clone());
    }
}

impl AddAssign<LineColor> for StyleContext {
    fn add_assign(&mut self, rhs: LineColor) {
        self.line_color = Some(rhs);
    }
}

impl AddAssign<&LineColor> for StyleContext {
    fn add_assign(&mut self, rhs: &LineColor) {
        self.line_color = Some(rhs.clone());
    }
}

impl AddAssign<TriangleEdgeWidth> for TriangleStyle {
    fn add_assign(&mut self, rhs: TriangleEdgeWidth) {
        self.triangle_edge_width = Some(rhs);
    }
}

impl AddAssign<&TriangleEdgeWidth> for TriangleStyle {
    fn add_assign(&mut self, rhs: &TriangleEdgeWidth) {
        self.triangle_edge_width = Some(rhs.clone());
    }
}

impl AddAssign<TriangleEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: TriangleEdgeWidth) {
        self.triangle_edge_width = Some(rhs);
    }
}

impl AddAssign<&TriangleEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &TriangleEdgeWidth) {
        self.triangle_edge_width = Some(rhs.clone());
    }
}

impl AddAssign<TriangleFillColor> for TriangleStyle {
    fn add_assign(&mut self, rhs: TriangleFillColor) {
        self.triangle_fill_color = Some(rhs);
    }
}

impl AddAssign<&TriangleFillColor> for TriangleStyle {
    fn add_assign(&mut self, rhs: &TriangleFillColor) {
        self.triangle_fill_color = Some(rhs.clone());
    }
}

impl AddAssign<TriangleFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: TriangleFillColor) {
        self.triangle_fill_color = Some(rhs);
    }
}

impl AddAssign<&TriangleFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: &TriangleFillColor) {
        self.triangle_fill_color = Some(rhs.clone());
    }
}

impl AddAssign<SquareEdgeWidth> for SquareStyle {
    fn add_assign(&mut self, rhs: SquareEdgeWidth) {
        self.square_edge_width = Some(rhs);
    }
}

impl AddAssign<&SquareEdgeWidth> for SquareStyle {
    fn add_assign(&mut self, rhs: &SquareEdgeWidth) {
        self.square_edge_width = Some(rhs.clone());
    }
}

impl AddAssign<SquareEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: SquareEdgeWidth) {
        self.square_edge_width = Some(rhs);
    }
}

impl AddAssign<&SquareEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &SquareEdgeWidth) {
        self.square_edge_width = Some(rhs.clone());
    }
}

impl AddAssign<SquareFillColor> for SquareStyle {
    fn add_assign(&mut self, rhs: SquareFillColor) {
        self.square_fill_color = Some(rhs);
    }
}

impl AddAssign<&SquareFillColor> for SquareStyle {
    fn add_assign(&mut self, rhs: &SquareFillColor) {
        self.square_fill_color = Some(rhs.clone());
    }
}

impl AddAssign<SquareFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: SquareFillColor) {
        self.square_fill_color = Some(rhs);
    }
}

impl AddAssign<&SquareFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: &SquareFillColor) {
        self.square_fill_color = Some(rhs.clone());
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

impl AddAssign<Self> for Point3DStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.point_size = rhs.point_size;
        self.point_color = rhs.point_color;
    }
}

impl AddAssign<&Self> for Point3DStyle {
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
        self.disk_fill_color = rhs.disk_fill_color;
        self.disk_edge_width = rhs.disk_edge_width;
        self.disk_edge_color = rhs.disk_edge_color;
    }
}

impl AddAssign<&Self> for DiskStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.disk_fill_color = rhs.disk_fill_color.clone();
        self.disk_edge_width = rhs.disk_edge_width.clone();
        self.disk_edge_color = rhs.disk_edge_color.clone();
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

impl AddAssign<Self> for TriangleStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.triangle_edge_width = rhs.triangle_edge_width;
        self.triangle_fill_color = rhs.triangle_fill_color;
    }
}

impl AddAssign<&Self> for TriangleStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.triangle_edge_width = rhs.triangle_edge_width.clone();
        self.triangle_fill_color = rhs.triangle_fill_color.clone();
    }
}

impl AddAssign<Self> for SquareStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.square_edge_width = rhs.square_edge_width;
        self.square_fill_color = rhs.square_fill_color;
    }
}

impl AddAssign<&Self> for SquareStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.square_edge_width = rhs.square_edge_width.clone();
        self.square_fill_color = rhs.square_fill_color.clone();
    }
}

impl AddAssign<Self> for RectangleStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.square_edge_width = rhs.square_edge_width;
        self.square_fill_color = rhs.square_fill_color;
    }
}

impl AddAssign<&Self> for RectangleStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.square_edge_width = rhs.square_edge_width.clone();
        self.square_fill_color = rhs.square_fill_color.clone();
    }
}
