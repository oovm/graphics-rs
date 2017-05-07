mod point;

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// The x-coordinate.
    pub x: f32,
    /// The y-coordinate.
    pub y: f32,
}
