use super::*;

impl Default for RGBA {
    fn default() -> Self {
        Self(Srgba::new(0.0, 0.0, 0.0, 1.0))
    }
}

impl PartialEq<Self> for RGBA {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for RGBA {}

impl From<(u8, u8, u8)> for RGBA {
    fn from(c: (u8, u8, u8)) -> Self {
        Self(Srgba::new(c.0 as f32 / 255.0, c.1 as f32 / 255.0, c.2 as f32 / 255.0, 1.0))
    }
}

impl From<(f32, f32, f32)> for RGBA {
    fn from(c: (f32, f32, f32)) -> Self {
        Self(Srgba::new(c.0, c.1, c.2, 1.0))
    }
}
