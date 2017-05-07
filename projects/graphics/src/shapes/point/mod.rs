use super::*;

impl Point {
    /// Construct new point
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    /// View point as tuple
    pub fn as_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
    /// Distance between two points.
    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
    /// TeX representation of point
    pub fn equation(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }
}

impl From<(f32, f32)> for Point {
    fn from(point: (f32, f32)) -> Self {
        Self { x: point.0, y: point.1 }
    }
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
