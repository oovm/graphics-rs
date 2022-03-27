use super::*;

impl AddAssign<BackgroundColor> for StyleContext {
    fn add_assign(&mut self, rhs: BackgroundColor) {
        self.background_color = Some(rhs);
    }
}

impl AddAssign<&BackgroundColor> for StyleContext {
    fn add_assign(&mut self, rhs: &BackgroundColor) {
        self.background_color = Some(rhs.clone());
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

impl AddAssign<PolygonEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: PolygonEdgeColor) {
        self.polygon_edge_color = Some(rhs);
    }
}

impl AddAssign<&PolygonEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: &PolygonEdgeColor) {
        self.polygon_edge_color = Some(rhs.clone());
    }
}

impl AddAssign<PolygonEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: PolygonEdgeWidth) {
        self.polygon_edge_width = Some(rhs);
    }
}

impl AddAssign<&PolygonEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &PolygonEdgeWidth) {
        self.polygon_edge_width = Some(rhs.clone());
    }
}

impl AddAssign<PolygonFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: PolygonFillColor) {
        self.polygon_fill_color = Some(rhs);
    }
}

impl AddAssign<&PolygonFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: &PolygonFillColor) {
        self.polygon_fill_color = Some(rhs.clone());
    }
}

impl AddAssign<PolylineColor> for StyleContext {
    fn add_assign(&mut self, rhs: PolylineColor) {
        self.polyline_color = Some(rhs);
    }
}

impl AddAssign<&PolylineColor> for StyleContext {
    fn add_assign(&mut self, rhs: &PolylineColor) {
        self.polyline_color = Some(rhs.clone());
    }
}

impl AddAssign<PolylineWidth> for StyleContext {
    fn add_assign(&mut self, rhs: PolylineWidth) {
        self.polyline_width = Some(rhs);
    }
}

impl AddAssign<&PolylineWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &PolylineWidth) {
        self.polyline_width = Some(rhs.clone());
    }
}

impl AddAssign<RectangleEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: RectangleEdgeColor) {
        self.rectangle_edge_color = Some(rhs);
    }
}

impl AddAssign<&RectangleEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: &RectangleEdgeColor) {
        self.rectangle_edge_color = Some(rhs.clone());
    }
}

impl AddAssign<RectangleEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: RectangleEdgeWidth) {
        self.rectangle_edge_width = Some(rhs);
    }
}

impl AddAssign<&RectangleEdgeWidth> for StyleContext {
    fn add_assign(&mut self, rhs: &RectangleEdgeWidth) {
        self.rectangle_edge_width = Some(rhs.clone());
    }
}

impl AddAssign<RectangleFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: RectangleFillColor) {
        self.rectangle_fill_color = Some(rhs);
    }
}

impl AddAssign<&RectangleFillColor> for StyleContext {
    fn add_assign(&mut self, rhs: &RectangleFillColor) {
        self.rectangle_fill_color = Some(rhs.clone());
    }
}

impl AddAssign<SquareEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: SquareEdgeColor) {
        self.square_edge_color = Some(rhs);
    }
}

impl AddAssign<&SquareEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: &SquareEdgeColor) {
        self.square_edge_color = Some(rhs.clone());
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

impl AddAssign<TextColor> for StyleContext {
    fn add_assign(&mut self, rhs: TextColor) {
        self.text_color = Some(rhs);
    }
}

impl AddAssign<&TextColor> for StyleContext {
    fn add_assign(&mut self, rhs: &TextColor) {
        self.text_color = Some(rhs.clone());
    }
}

impl AddAssign<TextFont> for StyleContext {
    fn add_assign(&mut self, rhs: TextFont) {
        self.text_font = Some(rhs);
    }
}

impl AddAssign<&TextFont> for StyleContext {
    fn add_assign(&mut self, rhs: &TextFont) {
        self.text_font = Some(rhs.clone());
    }
}

impl AddAssign<TextSize> for StyleContext {
    fn add_assign(&mut self, rhs: TextSize) {
        self.text_size = Some(rhs);
    }
}

impl AddAssign<&TextSize> for StyleContext {
    fn add_assign(&mut self, rhs: &TextSize) {
        self.text_size = Some(rhs.clone());
    }
}

impl AddAssign<TriangleEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: TriangleEdgeColor) {
        self.triangle_edge_color = Some(rhs);
    }
}

impl AddAssign<&TriangleEdgeColor> for StyleContext {
    fn add_assign(&mut self, rhs: &TriangleEdgeColor) {
        self.triangle_edge_color = Some(rhs.clone());
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

impl AddAssign<Self> for BackgroundStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.background_color = rhs.background_color;
    }
}

impl AddAssign<&Self> for BackgroundStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.background_color = rhs.background_color.clone();
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

impl AddAssign<Self> for EdgeColor {
    fn add_assign(&mut self, rhs: Self) {
        self.disk_edge_color = rhs.disk_edge_color;
        self.triangle_edge_color = rhs.triangle_edge_color;
        self.square_edge_color = rhs.square_edge_color;
        self.rectangle_edge_color = rhs.rectangle_edge_color;
        self.polygon_edge_color = rhs.polygon_edge_color;
    }
}

impl AddAssign<&Self> for EdgeColor {
    fn add_assign(&mut self, rhs: &Self) {
        self.disk_edge_color = rhs.disk_edge_color.clone();
        self.triangle_edge_color = rhs.triangle_edge_color.clone();
        self.square_edge_color = rhs.square_edge_color.clone();
        self.rectangle_edge_color = rhs.rectangle_edge_color.clone();
        self.polygon_edge_color = rhs.polygon_edge_color.clone();
    }
}

impl AddAssign<Self> for EdgeStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.disk_edge_width = rhs.disk_edge_width;
        self.triangle_edge_color = rhs.triangle_edge_color;
    }
}

impl AddAssign<&Self> for EdgeStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.disk_edge_width = rhs.disk_edge_width.clone();
        self.triangle_edge_color = rhs.triangle_edge_color.clone();
    }
}

impl AddAssign<Self> for EdgeWidth {
    fn add_assign(&mut self, rhs: Self) {
        self.disk_edge_width = rhs.disk_edge_width;
        self.triangle_edge_width = rhs.triangle_edge_width;
        self.square_edge_width = rhs.square_edge_width;
        self.rectangle_edge_width = rhs.rectangle_edge_width;
        self.polygon_edge_width = rhs.polygon_edge_width;
    }
}

impl AddAssign<&Self> for EdgeWidth {
    fn add_assign(&mut self, rhs: &Self) {
        self.disk_edge_width = rhs.disk_edge_width.clone();
        self.triangle_edge_width = rhs.triangle_edge_width.clone();
        self.square_edge_width = rhs.square_edge_width.clone();
        self.rectangle_edge_width = rhs.rectangle_edge_width.clone();
        self.polygon_edge_width = rhs.polygon_edge_width.clone();
    }
}

impl AddAssign<Self> for FillColor {
    fn add_assign(&mut self, rhs: Self) {
        self.disk_fill_color = rhs.disk_fill_color;
        self.triangle_fill_color = rhs.triangle_fill_color;
        self.square_fill_color = rhs.square_fill_color;
        self.rectangle_fill_color = rhs.rectangle_fill_color;
        self.polygon_fill_color = rhs.polygon_fill_color;
    }
}

impl AddAssign<&Self> for FillColor {
    fn add_assign(&mut self, rhs: &Self) {
        self.disk_fill_color = rhs.disk_fill_color.clone();
        self.triangle_fill_color = rhs.triangle_fill_color.clone();
        self.square_fill_color = rhs.square_fill_color.clone();
        self.rectangle_fill_color = rhs.rectangle_fill_color.clone();
        self.polygon_fill_color = rhs.polygon_fill_color.clone();
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

impl AddAssign<Self> for PolygonStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.polygon_fill_color = rhs.polygon_fill_color;
        self.polygon_edge_width = rhs.polygon_edge_width;
        self.polygon_edge_color = rhs.polygon_edge_color;
    }
}

impl AddAssign<&Self> for PolygonStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.polygon_fill_color = rhs.polygon_fill_color.clone();
        self.polygon_edge_width = rhs.polygon_edge_width.clone();
        self.polygon_edge_color = rhs.polygon_edge_color.clone();
    }
}

impl AddAssign<Self> for PolylineStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.polyline_width = rhs.polyline_width;
        self.polyline_color = rhs.polyline_color;
    }
}

impl AddAssign<&Self> for PolylineStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.polyline_width = rhs.polyline_width.clone();
        self.polyline_color = rhs.polyline_color.clone();
    }
}

impl AddAssign<Self> for RectangleStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.rectangle_fill_color = rhs.rectangle_fill_color;
        self.rectangle_edge_width = rhs.rectangle_edge_width;
        self.rectangle_edge_color = rhs.rectangle_edge_color;
    }
}

impl AddAssign<&Self> for RectangleStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.rectangle_fill_color = rhs.rectangle_fill_color.clone();
        self.rectangle_edge_width = rhs.rectangle_edge_width.clone();
        self.rectangle_edge_color = rhs.rectangle_edge_color.clone();
    }
}

impl AddAssign<Self> for SquareStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.square_fill_color = rhs.square_fill_color;
        self.square_edge_width = rhs.square_edge_width;
        self.square_edge_color = rhs.square_edge_color;
    }
}

impl AddAssign<&Self> for SquareStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.square_fill_color = rhs.square_fill_color.clone();
        self.square_edge_width = rhs.square_edge_width.clone();
        self.square_edge_color = rhs.square_edge_color.clone();
    }
}

impl AddAssign<Self> for TextStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.text_color = rhs.text_color;
        self.text_size = rhs.text_size;
        self.text_font = rhs.text_font;
    }
}

impl AddAssign<&Self> for TextStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.text_color = rhs.text_color.clone();
        self.text_size = rhs.text_size.clone();
        self.text_font = rhs.text_font.clone();
    }
}

impl AddAssign<Self> for TriangleStyle {
    fn add_assign(&mut self, rhs: Self) {
        self.triangle_fill_color = rhs.triangle_fill_color;
        self.triangle_edge_width = rhs.triangle_edge_width;
        self.triangle_edge_color = rhs.triangle_edge_color;
    }
}

impl AddAssign<&Self> for TriangleStyle {
    fn add_assign(&mut self, rhs: &Self) {
        self.triangle_fill_color = rhs.triangle_fill_color.clone();
        self.triangle_edge_width = rhs.triangle_edge_width.clone();
        self.triangle_edge_color = rhs.triangle_edge_color.clone();
    }
}
