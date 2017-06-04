use super::*;

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
