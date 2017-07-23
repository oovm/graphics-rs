use super::*;

impl Rectangle {
    // pub fn get_size(&self, style: &StyleResolver) -> f32 {
    //     self.size.unwrap_or(style.point_size()).clone().value
    // }
    //
    // pub fn set_size<T>(&mut self, value: T)
    // where
    //     T: Into<PointSize>,
    // {
    //     self.size = Some(value.into())
    // }

    pub fn get_color(&self, style: &StyleResolver) -> RGBA {
        self.color.unwrap_or(style.point_color()).clone()
    }

    pub fn set_color<T>(&mut self, value: T)
    where
        T: Into<RGBA>,
    {
        self.color = Some(value.into())
    }

    pub fn with_color<T>(mut self, value: T) -> Self
    where
        T: Into<RGBA>,
    {
        self.set_color(value);
        self
    }
}
