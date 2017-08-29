use super::*;

impl Point {
    /// Get the x-coordinate of the point
    pub fn get_x(&self) -> f32 {
        self.x
    }

    /// Get the y-coordinate of the point
    pub fn get_y(&self) -> f32 {
        self.y
    }


    /// Getter of [`PointSize`]
    pub fn get_size(&self, style: &StyleResolver) -> f32 {
        self.size.unwrap_or(style.point_size()).clone()
    }

    /// Setter of [`PointSize`]
    pub fn set_size<T>(&mut self, value: T)
    where
        T: Into<f32>,
    {
        self.size = Some(value.into())
    }

    /// Builder of [`PointSize`]
    pub fn with_size<T>(mut self, value: T) -> Self
    where
        T: Into<f32>,
    {
        self.set_size(value);
        self
    }
    /// Getter of [`PointColor`]
    pub fn get_color(&self, style: &StyleResolver) -> RGBA {
        self.color.unwrap_or(style.point_color()).clone()
    }

    /// Setter of [`PointColor`]
    pub fn set_color<T>(&mut self, value: T)
    where
        T: Into<RGBA>,
    {
        self.color = Some(value.into())
    }

    /// Builder of [`PointColor`]
    pub fn with_color<T>(mut self, value: T) -> Self
    where
        T: Into<RGBA>,
    {
        self.set_color(value);
        self
    }

}


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


impl Disk {
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
