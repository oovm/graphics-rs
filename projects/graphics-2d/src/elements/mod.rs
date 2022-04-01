use graphics_shape::{Circle, Ellipse, Point};
use graphics_style::{CircleStyle, Color, EllipseStyle, GraphicsStyle, PointStyle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pixel {
    pub x: f32,
    pub y: f32,
    pub c: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pixel3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub c: Color,
}

#[derive(Debug)]
pub enum GraphicEffect {
    StyleChange { style: Box<dyn GraphicsStyle>, finish: bool },
    Point { shape: Point, style: PointStyle, state: DrawableState },
    Circle { shape: Circle, style: CircleStyle, state: DrawableState },
    Ellipse { shape: Ellipse, style: EllipseStyle, state: DrawableState },
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
