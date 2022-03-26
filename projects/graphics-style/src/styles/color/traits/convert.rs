impl From<(u8, u8, u8)> for RGBA {
    fn from(c: (u8, u8, u8)) -> Self {
        Self { r: c.0, g: c.1, b: c.2, a: 255 }
    }
}

impl From<[u8; 3]> for RGBA {
    fn from(c: [u8; 3]) -> Self {
        Self { r: c[0], g: c[1], b: c[2], a: 255 }
    }
}

impl From<(u8, u8, u8, u8)> for RGBA {
    fn from(c: (u8, u8, u8, u8)) -> Self {
        Self { r: c.0, g: c.1, b: c.2, a: c.3 }
    }
}

impl From<[u8; 4]> for RGBA {
    fn from(c: [u8; 4]) -> Self {
        Self { r: c[0], g: c[1], b: c[2], a: c[3] }
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

impl From<RGBA> for u32 {
    fn from(c: RGBA) -> Self {
        u32::from_le_bytes([c.r, c.g, c.b, c.a])
    }
}

impl From<i32> for RGBA {
    fn from(c: i32) -> Self {
        let [r, g, b, a] = c.to_le_bytes();
        Self { r, g, b, a }
    }
}

impl From<u32> for RGBA {
    fn from(c: u32) -> Self {
        let [r, g, b, a] = c.to_le_bytes();
        Self { r, g, b, a }
    }
}
