mod dim2;
mod dim3;
use super::*;

/// A 2D point.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point {
    x: f32,
    y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Point3D {
    x: f32,
    y: f32,
    z: f32,
}
