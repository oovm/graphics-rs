use crate::{Ball, Float, Pi};
use num::{One, Zero};

impl<const N: usize> Ball<N> {
    /// N-dimensional unit ball
    pub fn unit(center: [Float; N]) -> Self {
        Self { center, radius: Float::one() }
    }
    /// Volume of N-dimensional ball.
    pub fn volume(&self) -> Float {
        let dim = self.center.len() as i32;
        let c = match dim % 2 {
            // pi^k/k!
            0 => Pi.powi(dim / 2) / factorial(dim / 2),
            // 2k!(4pi)^k/(2k+1)
            _ => 2.0 * factorial(dim / 2) * (4.0 * Pi).powi(dim / 2) / dim as Float,
        };
        c * self.radius.powi(dim)
    }
    /// Surface area of N-dimensional ball.
    pub fn surface_area(&self) -> Float {
        let dim = self.center.len() as Float;
        dim * self.volume() / self.radius
    }
    /// Check if point is inside the ball.
    pub fn contains_point(&self, point: [Float; N]) -> bool {
        let mut dist = Float::zero();
        for (x, y) in self.center.iter().zip(point.iter()) {
            dist += (x - y).powi(2);
        }
        dist <= self.radius.powi(2)
    }
}

fn factorial(n: i32) -> Float {
    assert!(n > 0);
    (1..=n).map(|i| i as Float).product()
}
