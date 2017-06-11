use crate::SVG;
use std::fmt::{Display, Formatter};

impl Display for SVG {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut indent = 0;
        self.write_tag_start(f, &mut indent)?;
        self.write_attributes(f, &mut indent)?;
        self.write_tag_end(f, &mut indent)?;
        self.write_children(f, &mut indent)?;
        self.write_final(f, &mut indent)?;
        Ok(())
    }
}

impl SVG {
    fn write_tag_start(&self, f: &mut Formatter<'_>, indent: &mut usize) -> std::fmt::Result {
        write!(f, "{:indent$}<{}", self.element, indent = indent)
    }

    fn write_attributes(&self, f: &mut Formatter<'_>, _: &mut usize) -> std::fmt::Result {
        for (k, v) in self.attributes.iter() {
            write!(f, " {}=\"{}\"", k, v)?;
        }
        Ok(())
    }

    fn write_tag_end(&self, f: &mut Formatter<'_>, _: &mut usize) -> std::fmt::Result {
        match self.children.is_empty() {
            true => writeln!(f, "/>"),
            false => writeln!(f, ">"),
        }
    }

    fn write_children(&self, f: &mut Formatter<'_>, indent: &mut usize) -> std::fmt::Result {
        *indent += 4;
        for child in self.children.iter() {
            write!(f, "{}", child)?;
        }
        *indent -= 4;
        Ok(())
    }

    fn write_final(&self, f: &mut Formatter<'_>, indent: &mut usize) -> std::fmt::Result {
        match self.children.is_empty() {
            true => writeln!(f, ""),
            false => writeln!(f, "{:indent$}</{}>", self.element, indent = indent),
        }
    }
}
