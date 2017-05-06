#![allow(non_upper_case_globals)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]

mod dim2;
mod dim_n;
mod traits;

pub use dim2::Circle;
pub use float::*;

#[cfg(feature = "f32")]
mod float {
    /// Float Type
    pub type Float = f32;
    /// constant π
    pub const Pi: Float = std::f32::consts::PI;
}

#[cfg(feature = "f64")]
mod float {
    /// Float Type
    pub type Float = f64;
    /// constant π
    pub const Pi: Float = std::f64::consts::PI;
}

/// A n-dimension point.
pub type Point<const N: usize> = [Float; N];

/// A ball in n-dimensional space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ball<const N: usize> {
    /// N dimensional center
    pub center: [Float; N],
    /// Radius of n-ball
    pub radius: Float,
}
