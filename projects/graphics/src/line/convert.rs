use super::*;

impl<T> From<(T, T, T)> for Ball<2>
where
    Float: From<T>,
{
    fn from(t: (T, T, T)) -> Self {
        Self { center: [Float::from(t.0), Float::from(t.1)], radius: Float::from(t.2) }
    }
}
