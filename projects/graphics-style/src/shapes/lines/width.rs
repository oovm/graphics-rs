use super::*;

impl PartialEq<f32> for LineWidth {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for LineWidth {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl From<f32> for LineWidth {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl From<i32> for LineWidth {
    fn from(v: i32) -> Self {
        Self { value: v as f32 }
    }
}
