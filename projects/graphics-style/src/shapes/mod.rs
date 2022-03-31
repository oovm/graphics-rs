// mod circle;
// mod lines;
// mod points;
// mod shape;
//
// use crate::{Color, Texture};
// #[allow(unused_imports)]
use serde::{Deserialize, Serialize};
// use setting::Setting;
// pub use shape::*;

use crate::{Color, Texture};

/// Get default style when not specified.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StyleContext {
    /// Get default [`CircleColor`] when missing.
    pub circle_fill_texture: Texture,

    /// Get default [`CircleWidth`] when missing.
    pub circle_edge_width: f32,

    /// Get default [`CircleWidth`] when missing.
    pub circle_edge_texture: Texture,
}

pub struct CircleStyle {
    pub fill_texture: Texture,
    pub edge_width: f32,
    pub edge_texture: Texture,
}

pub struct CircleFillTexture {
    texture: Texture,
}

pub struct CircleFillColor {
    color: Color,
}

pub struct CircleEdgeWidth {
    width: f32,
}
