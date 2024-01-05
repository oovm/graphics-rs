use super::*;


#[derive(Debug)]
pub struct Point {
    position: shape_core::Point<f32>,
    style: PointStyle,
}

#[derive(Debug, Default)]
pub struct PointStyle {
    pub color: StyleAttribute<Color>,
    pub size: StyleAttribute<f32>,
}




impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            position: shape_core::Point::new(x, y),
            style: PointStyle::default(),
        }
    }
}

impl Point {
    pub fn get_x(&self) -> f32 {
        self.position.x
    }
    pub fn set_x(&mut self, x: f32) {
        self.position.x = x;
    }
    pub fn mut_x(&mut self) -> &mut f32 {
        &mut self.position.x
    }
    pub fn with_x(mut self, x: f32) -> Self {
        self.position.x = x;
        self
    }
    pub fn get_y(&self) -> f32 {
        self.position.y
    }
    pub fn set_y(&mut self, y: f32) {
        self.position.y = y;
    }
    pub fn mut_y(&mut self) -> &mut f32 {
        &mut self.position.y
    }
    pub fn with_y(mut self, y: f32) -> Self {
        self.position.y = y;
        self
    }
    pub fn get_style(&self) -> &PointStyle {
        &self.style
    }
    pub fn set_style(&mut self, style: PointStyle) {
        self.style = style;
    }
    pub fn mut_style(&mut self) -> &mut PointStyle {
        &mut self.style
    }
    pub fn with_style(mut self, style: PointStyle) -> Self {
        self.style = style;
        self
    }
    pub fn get_color(&self, theme: &Option<GraphicTheme>) -> &Color {
        match self.style.color {
            StyleAttribute::Initial => {}
            StyleAttribute::Inherit => {}
            StyleAttribute::Static(_) => {}
            StyleAttribute::Custom(_) => {}
            StyleAttribute::Shared(_) => {}
        }
    }
}
