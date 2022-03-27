mod circle;
mod lines;
mod points;
mod shape;

#[allow(unused_imports)]
use crate::StyleResolver;
use crate::RGBA;
use serde::{Deserialize, Serialize};

pub use shape::*;
