use super::*;

impl GraphicsStyle3D for CircleStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.circle_edge_width += self.circle_width;
        state.circle_fill_texture += self.circle_texture;
    }
}

impl GraphicsStyle3D for CircleTexture {
    fn change_style(&self, state: &mut StyleContext) {
        state.circle_fill_texture = Some(self.clone());
    }
}

impl GraphicsStyle3D for CircleWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.circle_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle3D for DiskEdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for DiskEdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle3D for DiskFillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for LineColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.line_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for LineWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.clone());
    }
}

impl GraphicsStyle3D for PointColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.point_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for PointSize {
    fn change_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.clone());
    }
}

impl GraphicsStyle3D for PolygonEdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.polygon_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for PolygonEdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.polygon_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle3D for PolygonFillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.polygon_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for PolylineColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.polyline_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for PolylineWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.polyline_width = Some(self.clone());
    }
}

impl GraphicsStyle3D for RectangleEdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.rectangle_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for RectangleEdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.rectangle_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle3D for RectangleFillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.rectangle_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for SquareEdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.square_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for SquareEdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.square_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle3D for SquareFillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.square_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for TextColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.text_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for TextFont {
    fn change_style(&self, state: &mut StyleContext) {
        state.text_font = Some(self.clone());
    }
}

impl GraphicsStyle3D for TextSize {
    fn change_style(&self, state: &mut StyleContext) {
        state.text_size = Some(self.clone());
    }
}

impl GraphicsStyle3D for TriangleEdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.triangle_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for TriangleEdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.triangle_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle3D for TriangleFillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.triangle_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle3D for BackgroundStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.background_texture += self.background_color;
    }
}

impl GraphicsStyle3D for DiskStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.disk_fill_color.unwrap_or_default());
        state.disk_edge_width = Some(self.disk_edge_width.unwrap_or_default());
        state.disk_edge_color = Some(self.disk_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle3D for EdgeColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_edge_color = Some(self.disk_edge_color.unwrap_or_default());
        state.triangle_edge_color = Some(self.triangle_edge_color.unwrap_or_default());
        state.square_edge_color = Some(self.square_edge_color.unwrap_or_default());
        state.rectangle_edge_color = Some(self.rectangle_edge_color.unwrap_or_default());
        state.polygon_edge_color = Some(self.polygon_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle3D for EdgeStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.disk_edge_width.unwrap_or_default());
        state.triangle_edge_color = Some(self.triangle_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle3D for EdgeWidth {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.disk_edge_width.unwrap_or_default());
        state.triangle_edge_width = Some(self.triangle_edge_width.unwrap_or_default());
        state.square_edge_width = Some(self.square_edge_width.unwrap_or_default());
        state.rectangle_edge_width = Some(self.rectangle_edge_width.unwrap_or_default());
        state.polygon_edge_width = Some(self.polygon_edge_width.unwrap_or_default());
    }
}

impl GraphicsStyle3D for FillColor {
    fn change_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.disk_fill_color.unwrap_or_default());
        state.triangle_fill_color = Some(self.triangle_fill_color.unwrap_or_default());
        state.square_fill_color = Some(self.square_fill_color.unwrap_or_default());
        state.rectangle_fill_color = Some(self.rectangle_fill_color.unwrap_or_default());
        state.polygon_fill_color = Some(self.polygon_fill_color.unwrap_or_default());
    }
}

impl GraphicsStyle3D for LineStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.line_width.unwrap_or_default());
        state.line_color = Some(self.line_color.unwrap_or_default());
    }
}

impl GraphicsStyle3D for PointStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.point_size.unwrap_or_default());
        state.point_color = Some(self.point_color.unwrap_or_default());
    }
}

impl GraphicsStyle3D for PolygonStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.polygon_fill_color = Some(self.polygon_fill_color.unwrap_or_default());
        state.polygon_edge_width = Some(self.polygon_edge_width.unwrap_or_default());
        state.polygon_edge_color = Some(self.polygon_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle3D for PolylineStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.polyline_width = Some(self.polyline_width.unwrap_or_default());
        state.polyline_color = Some(self.polyline_color.unwrap_or_default());
    }
}

impl GraphicsStyle3D for RectangleStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.rectangle_fill_color = Some(self.rectangle_fill_color.unwrap_or_default());
        state.rectangle_edge_width = Some(self.rectangle_edge_width.unwrap_or_default());
        state.rectangle_edge_color = Some(self.rectangle_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle3D for SquareStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.square_fill_color = Some(self.square_fill_color.unwrap_or_default());
        state.square_edge_width = Some(self.square_edge_width.unwrap_or_default());
        state.square_edge_color = Some(self.square_edge_color.unwrap_or_default());
    }
}

impl GraphicsStyle3D for TextStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.text_color = Some(self.text_color.unwrap_or_default());
        state.text_size = Some(self.text_size.unwrap_or_default());
        state.text_font = Some(self.text_font.unwrap_or_default());
    }
}

impl GraphicsStyle3D for TriangleStyle {
    fn change_style(&self, state: &mut StyleContext) {
        state.triangle_fill_color = Some(self.triangle_fill_color.unwrap_or_default());
        state.triangle_edge_width = Some(self.triangle_edge_width.unwrap_or_default());
        state.triangle_edge_color = Some(self.triangle_edge_color.unwrap_or_default());
    }
}
