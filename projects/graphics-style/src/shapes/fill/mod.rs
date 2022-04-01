use super::*;

impl<T> AddAssign<T> for FillStyle
where
    T: Into<FillStyle>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = rhs.into();
    }
}

impl From<&Color> for FillStyle {
    #[inline(always)]
    fn from(c: &Color) -> Self {
        Self { texture: Texture::Color(*c) }
    }
}

impl From<Color> for FillStyle {
    #[inline(always)]
    fn from(c: Color) -> Self {
        Self { texture: Texture::Color(c) }
    }
}

impl From<&Self> for FillStyle {
    fn from(v: &Self) -> Self {
        v.clone()
    }
}
