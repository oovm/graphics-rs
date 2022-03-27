use crate::RGBA;

use std::cmp::Ordering;

mod circle;
mod disk;
mod lines;
mod points;

#[allow(unused_imports)]
use crate::StyleResolver;
use serde::{Deserialize, Serialize};

include!("shape.rs");
