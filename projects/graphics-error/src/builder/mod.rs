mod from_std;
use crate::{GraphicsError, GraphicsErrorKind};
use std::panic::Location;

impl GraphicsError {
    pub fn parse_error(message: String) -> Self {
        Self { kind: GraphicsErrorKind::ParseError(message), line: 0, column: 0, file: None }
    }
}

impl GraphicsErrorKind {
    fn with_caller(self, location: Location) -> GraphicsError {
        GraphicsError { kind: self, line: location.line(), column: location.column(), file: Some(location.file().to_string()) }
    }
}

#[macro_export]
macro_rules! parse_error {
    ($e:expr) => {
        Err(GraphicsError::parse_error($e.to_string()))
    };
    ($($arg:tt)*) => {
        Err(GraphicsError::parse_error(format!($($arg)*)))
    };
}
