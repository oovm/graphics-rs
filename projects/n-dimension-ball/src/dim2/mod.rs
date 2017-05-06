use crate::{Float, Pi};

/// A circle in 2D plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Circle {
    /// The center of the circle.
    pub center: (Float, Float),
    /// The radius of the circle.
    pub radius: Float,
}

impl Circle {
    /// Constructs a new unit circle.
    pub fn unit(x: Float, y: Float) -> Self {
        Self { center: (x, y), radius: 1.0 }
    }
    /// Create circle from center and radius.
    pub fn new(center: (Float, Float), radius: Float) -> Self {
        Self { center, radius }
    }
    /// Create circle from center and a point.
    pub fn from_2_points(center: (Float, Float), p: (Float, Float)) -> Self {
        todo!();
        Self { center, radius: 0.0 }
    }
    /// Create circle that intersects the 3 points.
    pub fn from_3_points(p1: (Float, Float), p2: (Float, Float), p3: (Float, Float)) -> Self {
        let cx = (x1 * c12 - p12.y * c13) / (2. * c123);
        let cy = (p12.x * c13 - p13.x * c12) / (2. * c123);

        let center = Point::new(cx + p1.x, cy + p1.y);

        Self { center: (0.0, 0.0), radius: 0.0 }
    }
}

impl Circle {
    pub fn equation(&self) -> (Float, Float, Float) {
        let (x, y) = self.center;
        let r = self.radius;
        (x, y, r)
    }

    /// Area of 2D ball.
    pub fn area(&self) -> Float {
        self.radius.powi(2) * Pi
    }
    /// Perimeter of 2D ball.
    pub fn perimeter(&self) -> Float {
        self.radius * Pi * 2.0
    }
    /// Distance between center and point.
    pub fn distance_to_center(&self, point: (Float, Float)) -> Float {
        distance(self.center, point)
    }
    /// Check if point is inside or on the circle.
    pub fn contains_point(&self, point: (Float, Float)) -> bool {
        distance(self.center, point) <= self.radius
    }
}

fn distance(a: (Float, Float), b: (Float, Float)) -> Float {
    let dx = b.0 - a.0;
    let dy = b.1 - a.1;
    (dx.powi(2) + dy.powi(2)).sqrt() - a.2
}
