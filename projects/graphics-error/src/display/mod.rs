use crate::{GraphicsError, GraphicsErrorKind};
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

impl Display for GraphicsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            GraphicsErrorKind::ParseError(e) => {
                writeln!(f, "Parse Error at: {}, {}", self.line, self.column)?;
                writeln!(f, "{:indent$}{msg}", " ", indent = 4, msg = e)
            }
            GraphicsErrorKind::IOError(e) => {
                match &self.file {
                    None => writeln!(f, "IO Error at: {}, {}", self.line, self.column)?,
                    Some(file) => writeln!(f, "IO Error at: {}, {} in {:?}", self.line, self.column, file)?,
                };
                writeln!(f, "{:indent$}{msg}", " ", indent = 4, msg = e)
            }
        }
    }
}

impl Error for GraphicsError {}
