use crate::SVG;
use std::fmt::{Display, Formatter};
use text_utils::indent;

impl Display for SVG {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut indent = 0;
        self.write_tag_start(f)?;
        self.write_attributes(f)?;
        self.write_children(f)?;
        self.write_tag_end(f)?;
        Ok(())
    }
}

impl SVG {
    fn write_tag_start(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}", self.element)
    }

    fn write_attributes(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (k, v) in self.attributes.iter() {
            write!(f, " {}=\"{}\"", k, v)?;
        }
        Ok(())
    }

    fn write_tag_end(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.children.is_empty() {
            true => write!(f, "/>"),
            false => write!(f, ">"),
        }
    }

    fn write_children(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for child in self.children.iter() {
            f.write_str(&indent(format!(f, "{}", child), 4))
        }
        Ok(())
    }

    fn write_final(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.children.is_empty() {
            true => write!(f, ""),
            false => write!(f, "</{}>", self.element),
        }
    }
}
