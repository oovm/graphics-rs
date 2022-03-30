impl Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(u32::from(*self))
    }
}

struct RGBAVisitor;

struct FieldVisitor;

enum RGBAField {
    Red,
    Green,
    Blue,
    Alpha,
}

impl<'de> Visitor<'de> for FieldVisitor {
    type Value = RGBAField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("`red` or `green` or `blue` or `alpha`")
    }

    fn visit_str<E>(self, value: &str) -> Result<RGBAField, E>
    where
        E: Error,
    {
        match value.to_ascii_lowercase().as_str() {
            "red" | "r" => Ok(RGBAField::Red),
            "green" | "g" => Ok(RGBAField::Green),
            "blue" | "b" => Ok(RGBAField::Blue),
            "alpha" | "a" => Ok(RGBAField::Alpha),
            _ => Err(Error::unknown_field(value, &["red", "green", "blue", "alpha"])),
        }
    }
}

impl<'de> Deserialize<'de> for RGBAField {
    fn deserialize<D>(deserializer: D) -> Result<RGBAField, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_identifier(FieldVisitor)
    }
}

impl<'de> Visitor<'de> for RGBAVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a hex string or a tuple of three or four integer between 0-255.")
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Color::from(v))
    }

    /// used for json
    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Color::from(v as u32))
    }
    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Color::from(v))
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(Color::from(v as u32))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let error = Err(Error::custom(format!("{} does not a valid hex color string", v)));
        if !v.starts_with("#") {
            return error;
        }
        fn hex_digit<E>(c: &u8) -> Result<u8, E>
        where
            E: Error,
        {
            match c {
                b'0'..=b'9' => Ok(c - b'0'),
                b'A'..=b'F' => Ok(c - b'A' + 10),
                b'a'..=b'f' => Ok(c - b'a' + 10),
                _ => Err(Error::custom(format!("{} does not a valid hex character", c))),
            }
        }
        let this = match &v[1..].as_bytes() {
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
        Ok(this)
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v {
            [r, g, b] => Ok(Color::new(*r, *g, *b, 255)),
            [r, g, b, a] => Ok(Color::new(*r, *g, *b, *a)),
            _ => Err(Error::custom(format!("RGBA must be a tuple of three or four integer between 0-255"))),
        }
    }

    fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        let _ = deserializer;
        todo!()
    }

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let _ = seq;
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

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(RGBAVisitor)
    }
}
