use super::*;

impl From<(u8, u8, u8)> for Color {
    fn from(c: (u8, u8, u8)) -> Self {
        Self::new(c.0, c.1, c.2, 255)
    }
}

impl From<[u8; 3]> for Color {
    fn from(c: [u8; 3]) -> Self {
        Self::new(c[0], c[1], c[2], 255)
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from(c: (u8, u8, u8, u8)) -> Self {
        Self::new(c.0, c.1, c.2, c.3)
    }
}

impl From<[u8; 4]> for Color {
    fn from(c: [u8; 4]) -> Self {
        Self::new(c[0], c[1], c[2], c[3])
    }
}

impl From<(f32, f32, f32)> for Color {
    fn from(c: (f32, f32, f32)) -> Self {
        Self::new(c.0 * 255.0 as u8, c.1 * 255.0 as u8, c.2 * 255.0 as u8, 255)
    }
}

impl From<(f32, f32, f32, f32)> for Color {
    fn from(c: (f32, f32, f32, f32)) -> Self {
        Self::new(c.0 * 255.0 as u8, c.1 * 255.0 as u8, c.2 * 255.0 as u8, c.3 * 255.0 as u8)
    }
}

impl From<Color> for u32 {
    fn from(c: Color) -> Self {
        let (r, g, b, a) = c.view();
        u32::from_le_bytes([r, g, b, a])
    }
}

impl From<u32> for Color {
    fn from(c: u32) -> Self {
        let [r, g, b, a] = c.to_le_bytes();
        Self::new(r, g, b, a)
    }
}

fn hex_digit(c: &u8) -> Result<u8> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => parse_error!("{} does not a valid hex character", c),
    }
}

impl FromStr for Color {
    type Err = GraphicsError;

    fn from_str(s: &str) -> Result<Self> {
        if !s.starts_with("#") {
            return parse_error!("{} does not a valid hex color string", s);
        }
        let this = match &s[1..].as_bytes() {
            [c1, c2] => {
                let c = hex_digit(c1)? * 16 + hex_digit(c2)?;
                Color::new(c, c, c, 255)
            }
            [r, g, b] => Color::new(
                //
                hex_digit(r)? * 17,
                hex_digit(g)? * 17,
                hex_digit(b)? * 17,
                255,
            ),
            [r, g, b, a] => Color::new(
                //
                hex_digit(r)? * 17,
                hex_digit(g)? * 17,
                hex_digit(b)? * 17,
                hex_digit(a)? * 17,
            ),
            [r1, r2, g1, g2, b1, b2] => Color::new(
                hex_digit(r1)? * 16 + hex_digit(r2)?,
                hex_digit(g1)? * 16 + hex_digit(g2)?,
                hex_digit(b1)? * 16 + hex_digit(b2)?,
                255,
            ),
            [r1, r2, g1, g2, b1, b2, a1, a2] => Color::new(
                hex_digit(r1)? * 16 + hex_digit(r2)?,
                hex_digit(g1)? * 16 + hex_digit(g2)?,
                hex_digit(b1)? * 16 + hex_digit(b2)?,
                hex_digit(a1)? * 16 + hex_digit(a2)?,
            ),
            _ => return parse_error!("Hex color string length must be 2, 3, 4, 6, 8."),
        };
        Ok(this)
    }
}
