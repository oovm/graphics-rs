use super::*;

impl GraphicsStyle for PointSize {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.value.clone());
    }
}

impl GraphicsStyle for PointSize {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.value.clone());
    }
}

impl GraphicsStyle for PointSize {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.value.clone());
    }
}

impl GraphicsStyle for PointSize {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.value.clone());
    }
}

impl GraphicsStyle for PointSize {
    fn draw_style(&self, state: &mut StyleContext) {
        state.point_size = Some(self.point_size.unwrap_or(PointSize::default().value).clone());
        state.point_color = Some(self.point_color.unwrap_or(PointColor::default().value).clone());
    }
}

impl GraphicsStyle for LineWidth {
    fn draw_style(&self, state: &mut StyleContext) {
        state.line_width = Some(self.line_width.unwrap_or(LineWidth::default().value).clone());
        state.line_color = Some(self.line_color.unwrap_or(LineColor::default().value).clone());
    }
}
