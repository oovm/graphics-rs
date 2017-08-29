use super::*;

impl Triangle {
    /// Get the 3 vertex of a triangle
    pub fn get_vertex(&self) -> [(f32, f32); 3] {
        self.vertex
    }

    /// Getter of [`Triangle`]
    pub fn get_color(&self, style: &StyleResolver) -> RGBA {
        self.color.unwrap_or(style.triangle_color()).clone()
    }

    /// Setter of [`Triangle`]
    pub fn set_color<T>(&mut self, value: T)
    where
        T: Into<RGBA>,
    {
        self.color = Some(value.into())
    }

    /// Builder of [`Triangle`]
    pub fn with_color<T>(mut self, value: T) -> Self
    where
        T: Into<RGBA>,
    {
        self.set_color(value);
        self
    }
}

impl Parallelogram {
    /// Get the 4 vertex of a parallelogram
    pub fn get_vertex(&self) -> [(f32, f32); 4] {
        self.vertex
    }

    /// Getter of [`Triangle`]
    pub fn get_color(&self, style: &StyleResolver) -> RGBA {
        self.color.unwrap_or(style.triangle_color()).clone()
    }

    /// Setter of [`Triangle`]
    pub fn set_color<T>(&mut self, value: T)
    where
        T: Into<RGBA>,
    {
        self.color = Some(value.into())
    }

    /// Builder of [`Triangle`]
    pub fn with_color<T>(mut self, value: T) -> Self
    where
        T: Into<RGBA>,
    {
        self.set_color(value);
        self
    }
}
