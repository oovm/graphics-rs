
use crate::*;


impl AddAssign<PointSize> for PointStyle {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs.value);
    }
}

impl AddAssign<&PointSize> for PointStyle {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs.value.clone());
    }
}

impl AddAssign<PointColor> for PointStyle {
    fn add_assign(&mut self, rhs: PointColor) {
        self.point_color = Some(rhs.value);
    }
}

impl AddAssign<&PointColor> for PointStyle {
    fn add_assign(&mut self, rhs: PointColor) {
        self.point_color = Some(rhs.value.clone());
    }
}


impl AddAssign<Self> for  {
    fn add_assign(&mut self, rhs: Self) {{self.point_size = rhs.point_size;
self.point_color = rhs.point_color;, self.point_size = rhs.point_size.clone();
self.point_color = rhs.point_color.clone();, PointStyle}}
}

impl AddAssign<&Self> for  {
    fn add_assign(&mut self, rhs: Self) {}
}

impl StyleResolver {
    /// Set the value of [`PointSize`]
    pub fn point_size(&self) -> PointStyle {
        self.local.point_size.unwrap_or(self.theme.point_size.unwrap_or(PointSize::default().value))
    }
    /// Set the value of [`PointColor`]
    pub fn point_color(&self) -> PointStyle {
        self.local.point_color.unwrap_or(self.theme.point_color.unwrap_or(PointColor::default().value))
    }
}

impl StyleResolver {
    /// Set the style of the given element.
    pub fn set_local_style<T>(&mut self, style: T)
    where
        T: Into<GraphicsStyle>,
    {
        match style.into() {
            GraphicsStyle::PointSize(s) => self.local.point_size = Some(s.clone()),
            GraphicsStyle::PointColor(s) => self.local.point_color = Some(s.clone()),
        }
    }
}

buildConvert[{PointStyle -> {{point_size, PointSize}, {point_color, PointColor}}}]