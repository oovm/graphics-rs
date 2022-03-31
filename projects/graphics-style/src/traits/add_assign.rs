use super::*;

impl<T> AddAssign<T> for StyleContext
where
    T: Into<Self>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        self.background_texture += rhs.background_texture
    }
}

impl AddAssign<CircleFillTexture> for CircleStyle {
    fn add_assign(&mut self, rhs: CircleFillTexture) {
        self.fill_texture = rhs.0;
    }
}
impl AddAssign<CircleFillColor> for CircleStyle {
    fn add_assign(&mut self, rhs: CircleFillColor) {
        self.fill_texture = Texture::Color(rhs.0);
    }
}
impl AddAssign<CircleFillTexture> for CircleStyle {
    fn add_assign(&mut self, rhs: CircleFillTexture) {
        self.fill_texture = rhs.0;
    }
}
