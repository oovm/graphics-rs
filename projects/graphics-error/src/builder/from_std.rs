use super::*;

impl From<std::io::Error> for GraphicsError {
    fn from(e: std::io::Error) -> Self {
        Self { kind: GraphicsErrorKind::IOError(e), line: 0, column: 0, file: None }
    }
}
