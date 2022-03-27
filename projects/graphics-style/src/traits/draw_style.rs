use super::*;

impl GraphicsStyle for PointSize {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.clone());
    }
}

impl GraphicsStyle for PointColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_color = Some(self.value.clone());
    }
}

impl GraphicsStyle for CircleWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.circle_width = Some(self.value.clone());
    }
}

impl GraphicsStyle for CircleColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.circle_color = Some(self.value.clone());
    }
}

impl GraphicsStyle for DiskFillColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.value.clone());
    }
}

impl GraphicsStyle for DiskEdgeWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_edge_width = Some(self.value.clone());
    }
}

impl GraphicsStyle for DiskEdgeColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_edge_color = Some(self.value.clone());
    }
}

impl GraphicsStyle for LineWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.value.clone());
    }
}

impl GraphicsStyle for LineColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.line_color = Some(self.value.clone());
    }
}

impl GraphicsStyle for PointStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = self.point_size.clone();
        state.point_color = Some(self.point_color.unwrap_or(PointColor::default().value).clone());
    }
}

impl GraphicsStyle for CircleStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.circle_width = Some(self.circle_width.unwrap_or(CircleWidth::default().value).clone());
        state.circle_color = Some(self.circle_color.unwrap_or(CircleColor::default().value).clone());
    }
}

impl GraphicsStyle for DiskStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.disk_fill_color = Some(self.disk_fill_color.unwrap_or(DiskFillColor::default().value).clone());
        state.disk_edge_width = Some(self.disk_edge_width.unwrap_or(DiskEdgeWidth::default().value).clone());
        state.disk_edge_color = Some(self.disk_edge_color.unwrap_or(DiskEdgeColor::default().value).clone());
    }
}

impl GraphicsStyle for LineStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.line_width.unwrap_or(LineWidth::default().value).clone());
        state.line_color = Some(self.line_color.unwrap_or(LineColor::default().value).clone());
    }
}
