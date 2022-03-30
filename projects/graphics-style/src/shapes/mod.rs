mod circle;
mod lines;
mod points;
mod shape;

use crate::Color;
#[allow(unused_imports)]
use crate::StyleResolver;
use serde::{Deserialize, Serialize};

pub use shape::*;
