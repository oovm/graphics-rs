use super::*;

/// Represent the available style of a circle.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub struct CircleStyle {
    ///
    pub circle_width: f32,
    ///
    pub circle_color: RGBA,
}
