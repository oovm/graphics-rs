use super::*;

impl GraphicsStyle for PointSize {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.clone());
    }
}

impl GraphicsStyle for PointColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_color = Some(self.clone());
    }
}

impl GraphicsStyle for CircleWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.circle_width = Some(self.clone());
    }
}

impl GraphicsStyle for CircleColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.circle_color = Some(self.clone());
    }
}

impl GraphicsStyle for DiskFillColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.clone());
    }
}

impl GraphicsStyle for DiskEdgeWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.clone());
    }
}

impl GraphicsStyle for DiskEdgeColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_edge_color = Some(self.clone());
    }
}

impl GraphicsStyle for LineWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.clone());
    }
}

impl GraphicsStyle for LineColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.line_color = Some(self.clone());
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

impl GraphicsStyle for PointStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.point_size.unwrap_or_default());
        state.point_color = Some(self.point_color.unwrap_or_default());
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

impl GraphicsStyle for LineStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.line_width.unwrap_or_default());
        state.line_color = Some(self.line_color.unwrap_or_default());
    }
}

impl GraphicsStyle for TriangleStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.triangle_edge_width = Some(self.triangle_edge_width.unwrap_or_default());
        state.triangle_fill_color = Some(self.triangle_fill_color.unwrap_or_default());
    }
}

impl GraphicsStyle for SquareStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.square_edge_width = Some(self.square_edge_width.unwrap_or_default());
        state.square_fill_color = Some(self.square_fill_color.unwrap_or_default());
    }
}
