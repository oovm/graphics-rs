use graphics_shape::{Circle, Ellipse};
use graphics_style::{CircleStyle, Color, EllipseStyle, GraphicsStyle, StyleAttribute};

mod point;

pub use self::point::{Point, PointStyle};

pub trait Drawable {
    fn draw(&self) -> GraphicElement;
}

pub trait Graphic {
    type Style;

    fn get_style(&self, theme: &Option<GraphicTheme>) -> &Self::Style;
}


#[derive(Debug)]
pub enum GraphicElement {
    StyleChange { style: Box<dyn GraphicsStyle>, finish: bool },
    Point(Point),
    Circle { shape: Circle, style: CircleStyle, state: DrawableState },
    Ellipse { shape: Ellipse, style: EllipseStyle, state: DrawableState },
    Sequences(Vec<GraphicElement>),
}


pub struct GraphicTheme {
    point: PointStyle,
}

#[derive(Debug)]
pub struct DrawableState {}

impl GraphicElement {
    pub fn change_style(&self) {}
}

impl GraphicElement {
    pub fn should_remove(&self) -> bool {
        todo!()
    }
}
