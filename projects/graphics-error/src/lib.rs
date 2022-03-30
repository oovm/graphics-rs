// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]missing_debug_implementations
#![doc = include_str!("../Readme.md")]

mod builder;
mod display;

pub type Result<T> = std::result::Result<T, GraphicsError>;

#[derive(Debug)]
pub struct GraphicsError {
    kind: GraphicsErrorKind,
    line: u32,
    column: u32,
    file: Option<String>,
}

#[derive(Debug)]
enum GraphicsErrorKind {
    ParseError(String),
}
