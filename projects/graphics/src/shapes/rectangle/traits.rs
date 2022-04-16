use super::*;

impl Default for Square {
    fn default() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0, width: 1.618, height: 1.0, color: None }
    }
}
