use super::*;

impl<T> AddAssign<T> for StyleContext
where
    T: Into<Self>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.background_color += rhs.background_color
    }
}

impl<T> AddAssign<T> for CircleWidth
where
    T: Into<Self>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.value = rhs.value;
    }
}

impl<T> AddAssign<T> for CircleTexture
where
    T: Into<Self>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.value = rhs.value;
    }
}

impl<T> AddAssign<T> for CircleStyle
where
    T: Into<Self>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.circle_width = rhs.circle_width;
        self.circle_texture = rhs.circle_texture;
    }
}
