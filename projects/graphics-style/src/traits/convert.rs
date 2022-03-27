use super::*;

impl From<f32> for PointSize {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl Into<f32> for PointSize {
    fn into(self) -> f32 {
        self.value
    }
}


impl From<RGBA> for PointColor {
    fn from(value: RGBA) -> Self {
        Self { value }
    }
}

impl Into<RGBA> for PointColor {
    fn into(self) -> RGBA {
        self.value
    }
}


impl From<f32> for CircleWidth {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl Into<f32> for CircleWidth {
    fn into(self) -> f32 {
        self.value
    }
}


impl From<RGBA> for CircleColor {
    fn from(value: RGBA) -> Self {
        Self { value }
    }
}

impl Into<RGBA> for CircleColor {
    fn into(self) -> RGBA {
        self.value
    }
}


impl From<RGBA> for DiskFillColor {
    fn from(value: RGBA) -> Self {
        Self { value }
    }
}

impl Into<RGBA> for DiskFillColor {
    fn into(self) -> RGBA {
        self.value
    }
}


impl From<f32> for DiskEdgeWidth {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl Into<f32> for DiskEdgeWidth {
    fn into(self) -> f32 {
        self.value
    }
}


impl From<RGBA> for DiskEdgeColor {
    fn from(value: RGBA) -> Self {
        Self { value }
    }
}

impl Into<RGBA> for DiskEdgeColor {
    fn into(self) -> RGBA {
        self.value
    }
}


impl From<f32> for LineWidth {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl Into<f32> for LineWidth {
    fn into(self) -> f32 {
        self.value
    }
}


impl From<RGBA> for LineColor {
    fn from(value: RGBA) -> Self {
        Self { value }
    }
}

impl Into<RGBA> for LineColor {
    fn into(self) -> RGBA {
        self.value
    }
}


impl PartialEq<f32> for PointSize {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for PointSize {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}


impl PartialEq<f32> for CircleWidth {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for CircleWidth {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}


impl PartialEq<f32> for DiskEdgeWidth {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for DiskEdgeWidth {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}


impl PartialEq<f32> for LineWidth {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for LineWidth {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}
