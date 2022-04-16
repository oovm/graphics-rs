use crate::SVG;
use std::fmt::{Display, Formatter};

impl Display for SVG {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut indent = 0;
        self.write_with_indent(f, &mut indent)?;
        Ok(())
    }
}

impl SVG {
    fn write_with_indent(&self, f: &mut Formatter<'_>, indent: &mut usize) -> std::fmt::Result {
        self.write_tag_start(f, indent)?;
        self.write_attributes(f, indent)?;
        self.write_tag_end(f, indent)?;
        self.write_children(f, indent)?;
        self.write_final(f, indent)
    }
    fn write_tag_start(&self, f: &mut Formatter<'_>, indent: &mut usize) -> std::fmt::Result {
        write!(f, "{:indent$}<{name}", " ", indent = indent, name = self.element)
    }

    fn write_attributes(&self, f: &mut Formatter<'_>, _: &mut usize) -> std::fmt::Result {
        for (k, v) in self.attributes.iter() {
            write!(f, " {}=\"{}\"", k, v)?;
        }
        Ok(())
    }

    fn write_tag_end(&self, f: &mut Formatter<'_>, _: &mut usize) -> std::fmt::Result {
        match self.children.is_empty() {
            true => write!(f, "/>"),
            false => writeln!(f, ">"),
        }
    }

    fn write_children(&self, f: &mut Formatter<'_>, indent: &mut usize) -> std::fmt::Result {
        *indent += 4;
        for child in self.children.iter() {
            child.write_with_indent(f, indent)?;
        }
        *indent -= 4;
        Ok(())
    }

    fn write_final(&self, f: &mut Formatter<'_>, indent: &mut usize) -> std::fmt::Result {
        match self.children.is_empty() {
            true => writeln!(f, ""),
            false => writeln!(f, "{:indent$}</{name}>", " ", indent = indent, name = self.element),
        }
    }
}
