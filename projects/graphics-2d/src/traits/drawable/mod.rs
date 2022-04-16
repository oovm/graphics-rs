use graphics_shape::{Circle, Ellipse, Point};
use graphics_style::{CircleStyle, EllipseStyle, GraphicsStyle, PointStyle};

pub trait Drawable {
    fn draw(&self) -> GraphicEffect;
}

#[derive(Debug)]
pub enum GraphicEffect {
    StyleChange { style: Box<dyn GraphicsStyle>, finish: bool },
    Point { shape: Point, style: PointStyle, state: DrawableState },
    Circle { shape: Circle, style: CircleStyle, state: DrawableState },
    Ellipse { shape: Ellipse, style: EllipseStyle, state: DrawableState },
    Sequences(Vec<GraphicEffect>),
}

#[derive(Debug)]

pub struct DrawableState {}

impl GraphicEffect {
    pub fn change_style(&self) {}
}

impl GraphicEffect {
    pub fn should_remove(&self) -> bool {
        todo!()
    }
}
