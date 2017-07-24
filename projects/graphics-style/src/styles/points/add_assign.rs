
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
    fn add_assign(&mut self, rhs: Self) {self.point_size = rhs.point_size;
self.point_color = rhs.point_color;}
}

impl AddAssign<&Self> for  {
    fn add_assign(&mut self, rhs: Self) {self.point_size = rhs.point_size.clone();
self.point_color = rhs.point_color.clone();}
}

impl AddAssign<PointSize> for StyleContext {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs.value);
    }
}
impl AddAssign<&PointSize> for StyleContext {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs.value);
    }
}

impl AddAssign<PointColor> for StyleContext {
    fn add_assign(&mut self, rhs: PointColor) {
        self.point_color = Some(rhs.value);
    }
}
impl AddAssign<&PointColor> for StyleContext {
    fn add_assign(&mut self, rhs: PointColor) {
        self.point_color = Some(rhs.value);
    }
}
