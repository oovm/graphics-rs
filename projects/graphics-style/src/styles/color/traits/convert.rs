use std::char::ParseCharError;
use std::num::ParseIntError;
use std::string::ParseError;

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

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Srgb::from_str()
        let this = match &s[1..].as_bytes() {
            [r, g, b] => Color::from([
                //
                hex_digit(r)? * 17,
                hex_digit(g)? * 17,
                hex_digit(b)? * 17,
            ]),
            [r, g, b, a] => Color::from([
                //
                hex_digit(r)? * 17,
                hex_digit(g)? * 17,
                hex_digit(b)? * 17,
                hex_digit(a)? * 17,
            ]),
            [r1, r2, g1, g2, b1, b2] => Color::from([
                hex_digit(r1)? * 16 + hex_digit(r2)?,
                hex_digit(g1)? * 16 + hex_digit(g2)?,
                hex_digit(b1)? * 16 + hex_digit(b2)?,
            ]),
            [r1, r2, g1, g2, b1, b2, a1, a2] => Color::from([
                hex_digit(r1)? * 16 + hex_digit(r2)?,
                hex_digit(g1)? * 16 + hex_digit(g2)?,
                hex_digit(b1)? * 16 + hex_digit(b2)?,
                hex_digit(a1)? * 16 + hex_digit(a2)?,
            ]),
            _ => return error,
        };

    }
}