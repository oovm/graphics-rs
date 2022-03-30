// #![deny(missing_docs)]
// #![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

pub struct GraphicsError {
    kind: GraphicsErrorKind,
}

pub enum GraphicsErrorKind {
    ParseError(String),
}
