use super::*;

impl Default for RGBA {
    fn default() -> Self {
        Self(Srgba::new(1.0, 1.0, 1.0, 1.0))
    }
}

impl PartialEq<Self> for RGBA {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl Eq for RGBA {}
