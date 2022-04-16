use super::*;

#[derive(Debug, Clone)]
pub struct PointStyle {
    pub color: Color,
    pub size: f32,
}

#[derive(Debug, Clone)]
pub struct PolygonStyle {
    pub fill: FillStyle,
    pub edge: EdgeStyle,
}

#[derive(Debug, Clone)]
pub struct RectangleStyle {
    pub fill: FillStyle,
    pub edge: EdgeStyle,
}

#[derive(Debug, Clone)]
pub struct EllipseStyle {
    pub fill: FillStyle,
    pub edge: EdgeStyle,
}

#[derive(Debug, Clone)]

pub struct CircleStyle {
    pub fill: FillStyle,
    pub edge: EdgeStyle,
}
#[derive(Debug, Clone)]

pub struct CircleWidth {}
#[derive(Debug, Clone)]

pub struct CircleColor {}

#[derive(Debug, Clone)]

pub struct FillStyle {
    pub texture: Texture,
}

#[derive(Debug, Clone)]

pub struct EdgeStyle {
    pub texture: Texture,
    pub width: f32,
}

#[derive(Debug, Clone)]

pub struct LineStyle {
    pub texture: Texture,
    pub width: f32,
}
