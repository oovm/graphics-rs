use super::*;

impl From<(f32, f32)> for Point {
    fn from(point: (f32, f32)) -> Self {
        Self { x: point.0 as f32, y: point.1 as f32, ..Default::default() }
    }
}
