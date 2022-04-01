use graphics_style::Color;
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
