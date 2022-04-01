use super::*;
impl From<&PolygonStyle> for RectangleStyle {
    fn from(style: &PolygonStyle) -> Self {
        Self { fill: style.fill.clone(), edge: style.edge.clone() }
    }
}
