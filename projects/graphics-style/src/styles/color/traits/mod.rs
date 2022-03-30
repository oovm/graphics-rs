use super::*;
use serde::de::Error;

include!("serde.rs");
include!("convert.rs");

impl Default for Color {
    fn default() -> Self {
        Self::new(0, 0, 0, 255)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (r, g, b, a) = self.view();
        write!(f, "rgba({}, {}, {}, {})", r, g, b, a)
    }
}

impl UpperHex for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_char('#')?;
        }
        let (r, g, b, a) = self.view();
        write!(f, "{:02X}{:02X}{:02X}{:02X}", r, g, b, a)
    }
}

impl LowerHex for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_char('#')?;
        }
        let (r, g, b, a) = self.view();
        write!(f, "{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
    }
}
