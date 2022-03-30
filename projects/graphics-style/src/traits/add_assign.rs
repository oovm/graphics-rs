use super::*;

impl<T> AddAssign<T> for StyleContext
where
    T: Into<Self>,
{
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        Self {
            background_color: self.background_color.or(rhs.background_color),
            circle_color: None,
            circle_width: None,
            disk_edge_color: None,
            disk_edge_width: None,
            disk_fill_color: None,
            line_color: None,
            line_width: None,
            point_color: None,
            point_size: None,
            polygon_edge_color: None,
            polygon_edge_width: None,
            polygon_fill_color: None,
            polyline_color: None,
            polyline_width: None,
            rectangle_edge_color: None,
            rectangle_edge_width: None,
            rectangle_fill_color: None,
            square_edge_color: None,
            square_edge_width: None,
            square_fill_color: None,
            text_color: None,
            text_font: None,
            text_size: None,
            triangle_edge_color: None,
            triangle_edge_width: None,
            triangle_fill_color: None,
        }
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
