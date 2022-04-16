use super::*;

impl From<(f32, f32, f32, f32)> for Line {
    fn from(v: (f32, f32, f32, f32)) -> Self {
        Self { x1: v.0, y1: v.1, x2: v.2, y2: v.3, ..Default::default() }
    }
}
