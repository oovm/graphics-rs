use crate::RGBA;

use std::cmp::Ordering;

mod circle;
mod disk;
mod lines;
mod points;

use serde::{Deserialize, Serialize};

include!("shape.rs");
