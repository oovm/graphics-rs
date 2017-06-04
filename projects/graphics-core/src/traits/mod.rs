pub trait Distance {
    type Other;
    fn distance(&self, other: &Self::Other) -> f32;
}
