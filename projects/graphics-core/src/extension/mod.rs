#[cfg(feature = "wolfram_wxf")]
mod wolfram;

/// A circle.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    /// Center of the circle.
    pub center: Point,
    /// Radius of the circle.
    pub radius: Float,
}

pub use shapes::Point;

/// The representation of a ellipse.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ellipse {
    a: Float,
    b: Float,
    c: Float,
    d: Float,
    e: Float,
    f: Float,
}
