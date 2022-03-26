use super::*;

impl Default for RGBA {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl From<(u8, u8, u8)> for RGBA {
    fn from(c: (u8, u8, u8)) -> Self {
        Self { r: c.0, g: c.1, b: c.2, a: 255 }
    }
}

impl From<(u8, u8, u8, u8)> for RGBA {
    fn from(c: (u8, u8, u8, u8)) -> Self {
        Self { r: c.0, g: c.1, b: c.2, a: c.3 }
    }
}

impl From<(f32, f32, f32)> for RGBA {
    fn from(c: (f32, f32, f32)) -> Self {
        Self { r: (c.0 * 255.0) as u8, g: (c.1 * 255.0) as u8, b: (c.2 * 255.0) as u8, a: 255 }
    }
}

impl From<(f32, f32, f32, f32)> for RGBA {
    fn from(c: (f32, f32, f32, f32)) -> Self {
        Self { r: (c.0 * 255.0) as u8, g: (c.1 * 255.0) as u8, b: (c.2 * 255.0) as u8, a: (c.3 * 255.0) as u8 }
    }
}
