use super::*;
use serde::{
    de::{EnumAccess, Error, MapAccess, SeqAccess, Visitor},
    Deserializer, Serializer,
};

impl Default for RGBA {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl Display for RGBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.0.red, self.0.green, self.0.blue, self.0.alpha)
    }
}

impl UpperHex for RGBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (r, g, b, a) = self.view();
        if f.alternate() {
            f.write_char('#')?;
        }
        write!(f, "{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }
}

impl LowerHex for RGBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (r, g, b, a) = self.view();
        if f.alternate() {
            f.write_char('#')?;
        }
        write!(f, "{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
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

impl Serialize for RGBA {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(u32::from(*self))
    }
}

struct RGBAVisitor;

impl<'de> Visitor<'de> for RGBAVisitor {
    type Value = RGBA;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an integer between -2^31 and 2^31")
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RGBA::from(v))
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(RGBA::from(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        if !v.starts_with("#") {
            return Err(E::custom(format!("expected hex color, got {}", v)));
        }
        let v: Vec<_> = v.chars().skip(1).take(8).collect();
        match v.as_slice() {
            [r, g, b] => {
                let r = u8::from_str_radix(r, 16).map_err(|_| E::custom(format!("invalid red hex value {}", r)))?;
                let g = u8::from_str_radix(g, 16).map_err(|_| E::custom(format!("invalid green hex value {}", g)))?;
                let b = u8::from_str_radix(b, 16).map_err(|_| E::custom(format!("invalid blue hex value {}", b)))?;
                Ok(RGBA::from((r, g, b)))
            }
            [r, g, b, a] => {
                let r = u8::from_str_radix(r, 16).map_err(|_| E::custom(format!("invalid red hex value {}", r)))?;
                let g = u8::from_str_radix(g, 16).map_err(|_| E::custom(format!("invalid green hex value {}", g)))?;
                let b = u8::from_str_radix(b, 16).map_err(|_| E::custom(format!("invalid blue hex value {}", b)))?;
                let a = u8::from_str_radix(a, 16).map_err(|_| E::custom(format!("invalid alpha hex value {}", a)))?;
                Ok(RGBA::from((r, g, b, a)))
            }
            [r1, r2, g1, g2, b1, b2] => {}
        }

        Ok(RGBA::from_hex(v))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v {
            [r, g, b] => Ok(RGBA::new(*r, *g, *b, 255)),
            [r, g, b, a] => Ok(RGBA::new(*r, *g, *b, *a)),
            _ => {}
        }
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        todo!()
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let _ = map;
        todo!()
    }
}

impl<'de> Deserialize<'de> for RGBA {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(RGBAVisitor)
    }
}

fn hex_digit(c: u8) -> Result<u8, ErrorMessage> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => nom_error(ErrorKind::HexDigit, "Invalid hex digit"),
    }
}
