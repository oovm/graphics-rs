use super::*;

impl Default for PointSize {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl PartialEq<f32> for PointSize {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for PointSize {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl From<f32> for PointSize {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl From<i32> for PointSize {
    fn from(v: i32) -> Self {
        Self { value: v as f32 }
    }
}
