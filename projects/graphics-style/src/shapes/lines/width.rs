use super::*;

impl From<i32> for LineWidth {
    fn from(v: i32) -> Self {
        Self { value: v as f32 }
    }
}
