use super::*;

impl From<&RectangleStyle> for PolygonStyle {
    fn from(style: &RectangleStyle) -> Self {
        Self { fill: style.fill.clone(), edge: style.edge.clone() }
    }
}

impl From<CircleStyle> for EllipseStyle {
    fn from(style: CircleStyle) -> Self {
        Self { fill: style.fill, edge: style.edge }
    }
}
