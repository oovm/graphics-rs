use super::*;

impl Default for CircleSize {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl PartialEq<f32> for CircleSize {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for CircleSize {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl From<f32> for CircleSize {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl From<i32> for CircleSize {
    fn from(v: i32) -> Self {
        Self { value: v as f32 }
    }
}
