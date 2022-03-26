use super::*;
use serde::de::Error;

include!("serde.rs");
include!("convert.rs");

impl Default for RGBA {
    fn default() -> Self {
        Self { r: 0, g: 0, b: 0, a: 255 }
    }
}

impl Display for RGBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

impl UpperHex for RGBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_char('#')?;
        }
        write!(f, "{:02X}{:02X}{:02X}{:02X}", self.r, self.g, self.b, self.a)
    }
}

impl LowerHex for RGBA {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_char('#')?;
        }
        write!(f, "{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, self.a)
    }
}