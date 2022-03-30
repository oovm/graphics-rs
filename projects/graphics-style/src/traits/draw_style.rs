use super::*;

impl GraphicsStyle for BackgroundColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.background_color = Some(self.clone());
    }
}

impl GraphicsStyle for CircleColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.circle_color = Some(self.clone());
    }
}

impl GraphicsStyle for CircleWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.circle_width = Some(self.clone());
    }
}

impl GraphicsStyle for DiskEdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle for DiskEdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle for DiskFillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle for LineColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.line_color = Some(self.clone());
    }
}

impl GraphicsStyle for LineWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.clone());
    }
}

impl GraphicsStyle for PointColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.point_color = Some(self.clone());
    }
}

impl GraphicsStyle for PointSize {
    fn change_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.clone());
    }
}

impl GraphicsStyle for PolygonEdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.polygon_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle for PolygonEdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.polygon_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle for PolygonFillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.polygon_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle for PolylineColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.polyline_color = Some(self.clone());
    }
}

impl GraphicsStyle for PolylineWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.polyline_width = Some(self.clone());
    }
}

impl GraphicsStyle for RectangleEdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.rectangle_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle for RectangleEdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.rectangle_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle for RectangleFillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.rectangle_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle for SquareEdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.square_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle for SquareEdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.square_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle for SquareFillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.square_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle for TextColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.text_color = Some(self.clone());
    }
}

impl GraphicsStyle for TextFont {
    fn change_style(&self, state: &mut StyleContext) {
        state.text_font = Some(self.clone());
    }
}

impl GraphicsStyle for TextSize {
    fn change_style(&self, state: &mut StyleContext) {
        state.text_size = Some(self.clone());
    }
}

impl GraphicsStyle for TriangleEdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.triangle_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle for TriangleEdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.triangle_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle for TriangleFillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.triangle_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle for BackgroundStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.background_color = Some(self.background_color.unwrap_or_default());
    }
}

impl GraphicsStyle for CircleStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.circle_width = Some(self.circle_width.unwrap_or_default());
        state.circle_color = Some(self.circle_color.unwrap_or_default());
    }
}

impl GraphicsStyle for DiskStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.disk_fill_color.unwrap_or_default());
        state.disk_edge_width = Some(self.disk_edge_width.unwrap_or_default());
        state.disk_edge_color = Some(self.disk_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for EdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_edge_color = Some(self.disk_edge_color.unwrap_or_default());
        state.triangle_edge_color = Some(self.triangle_edge_color.unwrap_or_default());
        state.square_edge_color = Some(self.square_edge_color.unwrap_or_default());
        state.rectangle_edge_color = Some(self.rectangle_edge_color.unwrap_or_default());
        state.polygon_edge_color = Some(self.polygon_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for EdgeStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.disk_edge_width.unwrap_or_default());
        state.triangle_edge_color = Some(self.triangle_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for EdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.disk_edge_width.unwrap_or_default());
        state.triangle_edge_width = Some(self.triangle_edge_width.unwrap_or_default());
        state.square_edge_width = Some(self.square_edge_width.unwrap_or_default());
        state.rectangle_edge_width = Some(self.rectangle_edge_width.unwrap_or_default());
        state.polygon_edge_width = Some(self.polygon_edge_width.unwrap_or_default());
    }
}

impl GraphicsStyle for FillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.disk_fill_color.unwrap_or_default());
        state.triangle_fill_color = Some(self.triangle_fill_color.unwrap_or_default());
        state.square_fill_color = Some(self.square_fill_color.unwrap_or_default());
        state.rectangle_fill_color = Some(self.rectangle_fill_color.unwrap_or_default());
        state.polygon_fill_color = Some(self.polygon_fill_color.unwrap_or_default());
    }
}

impl GraphicsStyle for LineStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.line_width.unwrap_or_default());
        state.line_color = Some(self.line_color.unwrap_or_default());
    }
}

impl GraphicsStyle for PointStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.point_size.unwrap_or_default());
        state.point_color = Some(self.point_color.unwrap_or_default());
    }
}

impl GraphicsStyle for PolygonStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.polygon_fill_color = Some(self.polygon_fill_color.unwrap_or_default());
        state.polygon_edge_width = Some(self.polygon_edge_width.unwrap_or_default());
        state.polygon_edge_color = Some(self.polygon_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for PolylineStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.polyline_width = Some(self.polyline_width.unwrap_or_default());
        state.polyline_color = Some(self.polyline_color.unwrap_or_default());
    }
}

impl GraphicsStyle for RectangleStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.rectangle_fill_color = Some(self.rectangle_fill_color.unwrap_or_default());
        state.rectangle_edge_width = Some(self.rectangle_edge_width.unwrap_or_default());
        state.rectangle_edge_color = Some(self.rectangle_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for SquareStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.square_fill_color = Some(self.square_fill_color.unwrap_or_default());
        state.square_edge_width = Some(self.square_edge_width.unwrap_or_default());
        state.square_edge_color = Some(self.square_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle for TextStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.text_color = Some(self.text_color.unwrap_or_default());
        state.text_size = Some(self.text_size.unwrap_or_default());
        state.text_font = Some(self.text_font.unwrap_or_default());
    }
}

impl GraphicsStyle for TriangleStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.triangle_fill_color = Some(self.triangle_fill_color.unwrap_or_default());
        state.triangle_edge_width = Some(self.triangle_edge_width.unwrap_or_default());
        state.triangle_edge_color = Some(self.triangle_edge_color.unwrap_or_default());
    }
}
