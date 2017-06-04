use super::*;

impl Default for RGBA {
    fn default() -> Self {
        Self(Srgba::new(1.0, 1.0, 1.0, 1.0))
    }
}
