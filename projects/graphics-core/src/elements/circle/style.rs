use super::*;

impl Circle {
    /// Get the x-coordinate of the circle center
    pub fn get_x(&self) -> f32 {
        self.x
    }

    /// Get the y-coordinate of the circle center
    pub fn get_y(&self) -> f32 {
        self.y
    }

    /// Get the radius of the circle
    pub fn get_radius(&self) -> f32 {
        self.radius
    }


    /// Getter of [`CircleColor`]
    pub fn get_color(&self, style: &StyleResolver) -> RGBA {
        self.color.unwrap_or(style.circle_color()).clone()
    }

    /// Setter of [`CircleColor`]
    pub fn set_color<T>(&mut self, value: T)
    where
        T: Into<RGBA>,
    {
        self.color = Some(value.into())
    }

    /// Builder of [`CircleColor`]
    pub fn with_color<T>(mut self, value: T) -> Self
    where
        T: Into<RGBA>,
    {
        self.set_color(value);
        self
    }

}
