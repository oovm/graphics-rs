use super::*;

impl GraphicsStyle for PointSize {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.value.clone());
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

impl GraphicsStyle for FillColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.fill_color = Some(self.value.clone());
    }
}

impl GraphicsStyle for EdgeWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.edge_width = Some(self.value.clone());
    }
}

impl GraphicsStyle for EdgeColor {
    fn draw_style(&self, state: &mut StyleContext) {
        state.edge_color = Some(self.value.clone());
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
        state.point_size = Some(self.point_size.unwrap_or(PointSize::default().value).clone());
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
        state.fill_color = Some(self.fill_color.unwrap_or(FillColor::default().value).clone());
        state.edge_width = Some(self.edge_width.unwrap_or(EdgeWidth::default().value).clone());
        state.edge_color = Some(self.edge_color.unwrap_or(EdgeColor::default().value).clone());
    }
}

impl GraphicsStyle for LineStyle {
    fn draw_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.line_width.unwrap_or(LineWidth::default().value).clone());
        state.line_color = Some(self.line_color.unwrap_or(LineColor::default().value).clone());
    }
}
