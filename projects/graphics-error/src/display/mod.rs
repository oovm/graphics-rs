use crate::GraphicsError;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

impl Display for GraphicsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for GraphicsError {}
