use super::*;

impl GraphicsStyle for BackgroundColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.background_color = Some(self.clone());
    }
}

impl GraphicsStyle for CircleColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.circle_color = Some(self.clone());
    }
}

impl GraphicsStyle for CircleWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.circle_width = Some(self.clone());
    }
}

impl GraphicsStyle for DiskEdgeColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle for DiskEdgeWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle for DiskFillColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle for LineColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.line_color = Some(self.clone());
    }
}

impl GraphicsStyle for LineWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.clone());
    }
}

impl GraphicsStyle for PointColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_color = Some(self.clone());
    }
}

impl GraphicsStyle for PointSize {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.clone());
    }
}

impl GraphicsStyle for PolygonEdgeColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.polygon_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle for PolygonEdgeWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.polygon_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle for PolygonFillColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.polygon_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle for PolylineColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.polyline_color = Some(self.clone());
    }
}

impl GraphicsStyle for PolylineWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.polyline_width = Some(self.clone());
    }
}

impl GraphicsStyle for RectangleEdgeColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.rectangle_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle for RectangleEdgeWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.rectangle_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle for RectangleFillColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.rectangle_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle for SquareEdgeColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.square_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle for SquareEdgeWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.square_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle for SquareFillColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.square_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle for TextColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.text_color = Some(self.clone());
    }
}

impl GraphicsStyle for TextFont {
    fn draw_style(&self, state: &mut StyleContext) {
        state.text_font = Some(self.clone());
    }
}

impl GraphicsStyle for TextSize {
    fn draw_style(&self, state: &mut StyleContext) {
        state.text_size = Some(self.clone());
    }
}

impl GraphicsStyle for TriangleEdgeColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.triangle_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle for TriangleEdgeWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.triangle_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle for TriangleFillColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.triangle_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle for BackgroundStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.background_color = Some(self.background_color.unwrap_or_default());
    }
}

impl GraphicsStyle for CircleStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.circle_width = Some(self.circle_width.unwrap_or_default());
        state.circle_color = Some(self.circle_color.unwrap_or_default());
    }
}

impl GraphicsStyle for DiskStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.disk_fill_color.unwrap_or_default());
        state.disk_edge_width = Some(self.disk_edge_width.unwrap_or_default());
        state.disk_edge_color = Some(self.disk_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for EdgeColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_edge_color = Some(self.disk_edge_color.unwrap_or_default());
        state.triangle_edge_color = Some(self.triangle_edge_color.unwrap_or_default());
        state.square_edge_color = Some(self.square_edge_color.unwrap_or_default());
        state.rectangle_edge_color = Some(self.rectangle_edge_color.unwrap_or_default());
        state.polygon_edge_color = Some(self.polygon_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for EdgeStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.disk_edge_width.unwrap_or_default());
        state.triangle_edge_color = Some(self.triangle_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for EdgeWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.disk_edge_width.unwrap_or_default());
        state.triangle_edge_width = Some(self.triangle_edge_width.unwrap_or_default());
        state.square_edge_width = Some(self.square_edge_width.unwrap_or_default());
        state.rectangle_edge_width = Some(self.rectangle_edge_width.unwrap_or_default());
        state.polygon_edge_width = Some(self.polygon_edge_width.unwrap_or_default());
    }
}

impl GraphicsStyle for FillColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.disk_fill_color.unwrap_or_default());
        state.triangle_fill_color = Some(self.triangle_fill_color.unwrap_or_default());
        state.square_fill_color = Some(self.square_fill_color.unwrap_or_default());
        state.rectangle_fill_color = Some(self.rectangle_fill_color.unwrap_or_default());
        state.polygon_fill_color = Some(self.polygon_fill_color.unwrap_or_default());
    }
}

impl GraphicsStyle for LineStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.line_width.unwrap_or_default());
        state.line_color = Some(self.line_color.unwrap_or_default());
    }
}

impl GraphicsStyle for PointStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.point_size.unwrap_or_default());
        state.point_color = Some(self.point_color.unwrap_or_default());
    }
}

impl GraphicsStyle for PolygonStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.polygon_fill_color = Some(self.polygon_fill_color.unwrap_or_default());
        state.polygon_edge_width = Some(self.polygon_edge_width.unwrap_or_default());
        state.polygon_edge_color = Some(self.polygon_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for PolylineStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.polyline_width = Some(self.polyline_width.unwrap_or_default());
        state.polyline_color = Some(self.polyline_color.unwrap_or_default());
    }
}

impl GraphicsStyle for RectangleStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.rectangle_fill_color = Some(self.rectangle_fill_color.unwrap_or_default());
        state.rectangle_edge_width = Some(self.rectangle_edge_width.unwrap_or_default());
        state.rectangle_edge_color = Some(self.rectangle_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for SquareStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.square_fill_color = Some(self.square_fill_color.unwrap_or_default());
        state.square_edge_width = Some(self.square_edge_width.unwrap_or_default());
        state.square_edge_color = Some(self.square_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for TextStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.text_color = Some(self.text_color.unwrap_or_default());
        state.text_size = Some(self.text_size.unwrap_or_default());
        state.text_font = Some(self.text_font.unwrap_or_default());
    }
}

impl GraphicsStyle for TriangleStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.triangle_fill_color = Some(self.triangle_fill_color.unwrap_or_default());
        state.triangle_edge_width = Some(self.triangle_edge_width.unwrap_or_default());
        state.triangle_edge_color = Some(self.triangle_edge_color.unwrap_or_default());
    }
}
