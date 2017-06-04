use super::*;

impl Point {
    pub fn get_size(&self, style: &StyleResolver) -> PointSize {
        self.size.unwrap_or(style.point_size()).clone()
    }

    pub fn set_size<T>(&mut self, value: T)
    where
        T: Into<PointSize>,
    {
        self.size = Some(value.into())
    }

    pub fn get_color(&self, style: &StyleResolver) -> RGBA {
        self.color.unwrap_or(style.point_color()).clone()
    }

    pub fn set_color<T>(&mut self, value: T)
    where
        T: Into<RGBA>,
    {
        self.color = Some(value.into())
    }
}
