use super::*;


impl AddAssign<PointSize> for  {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs.value);
    }
}

impl AddAssign<&PointSize> for  {
    fn add_assign(&mut self, rhs: PointSize) {
        self.point_size = Some(rhs.value.clone());
    }
}


impl AddAssign<PointColor> for  {
    fn add_assign(&mut self, rhs: PointColor) {
        self.point_color = Some(rhs.value);
    }
}

impl AddAssign<&PointColor> for  {
    fn add_assign(&mut self, rhs: PointColor) {
        self.point_color = Some(rhs.value.clone());
    }
}


getField[typeSuper -> PointStyle]

impl AddAssign<LineWidth> for  {
    fn add_assign(&mut self, rhs: LineWidth) {
        self.line_width = Some(rhs.value);
    }
}

impl AddAssign<&LineWidth> for  {
    fn add_assign(&mut self, rhs: LineWidth) {
        self.line_width = Some(rhs.value.clone());
    }
}


impl AddAssign<LineColor> for  {
    fn add_assign(&mut self, rhs: LineColor) {
        self.line_color = Some(rhs.value);
    }
}

impl AddAssign<&LineColor> for  {
    fn add_assign(&mut self, rhs: LineColor) {
        self.line_color = Some(rhs.value.clone());
    }
}


getField[typeSuper -> LineStyle]

impl AddAssign<Self> for {<|field -> point_size, typeOuter -> PointSize, typeInner -> f64|>, <|field -> point_color, typeOuter -> PointColor, typeInner -> RGBA|>, typeSuper -> PointStyle} {
    fn add_assign(&mut self, rhs: Self) {{{<|field -> point_size, typeOuter -> PointSize, typeInner -> f64|>, <|field -> point_color, typeOuter -> PointColor, typeInner -> RGBA|>, typeSuper -> PointStyle}, {<|field -> line_width, typeOuter -> LineWidth, typeInner -> f64|>, <|field -> line_color, typeOuter -> LineColor, typeInner -> RGBA|>, typeSuper -> LineStyle}, typeSuper -> getField1}}
}

impl AddAssign<&Self> for {<|field -> point_size, typeOuter -> PointSize, typeInner -> f64|>, <|field -> point_color, typeOuter -> PointColor, typeInner -> RGBA|>, typeSuper -> PointStyle} {
    fn add_assign(&mut self, rhs: Self) {{{<|field -> point_size, typeOuter -> PointSize, typeInner -> f64|>, <|field -> point_color, typeOuter -> PointColor, typeInner -> RGBA|>, typeSuper -> PointStyle}, {<|field -> line_width, typeOuter -> LineWidth, typeInner -> f64|>, <|field -> line_color, typeOuter -> LineColor, typeInner -> RGBA|>, typeSuper -> LineStyle}, typeSuper -> getField2}}
}

{<|field -> point_size, typeOuter -> PointSize, typeInner -> f64|>, <|field -> point_color, typeOuter -> PointColor, typeInner -> RGBA|>, typeSuper -> PointStyle}

{{<|field -> point_size, typeOuter -> PointSize, typeInner -> f64|>, <|field -> point_color, typeOuter -> PointColor, typeInner -> RGBA|>, typeSuper -> PointStyle}, {<|field -> line_width, typeOuter -> LineWidth, typeInner -> f64|>, <|field -> line_color, typeOuter -> LineColor, typeInner -> RGBA|>, typeSuper -> LineStyle}, typeSuper -> getField1}{{<|field -> point_size, typeOuter -> PointSize, typeInner -> f64|>, <|field -> point_color, typeOuter -> PointColor, typeInner -> RGBA|>, typeSuper -> PointStyle}, {<|field -> line_width, typeOuter -> LineWidth, typeInner -> f64|>, <|field -> line_color, typeOuter -> LineColor, typeInner -> RGBA|>, typeSuper -> LineStyle}, typeSuper -> getField2}