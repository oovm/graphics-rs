mod disk;
mod line;
mod pixel;
mod polygon;
mod rectangle;

use graphics_style::{StyleResolver, RGBA};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pixel {
    pub x: f32,
    pub y: f32,
    pub c: RGBA,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Pixel3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub c: RGBA,
}

include!("shapes.rs");
