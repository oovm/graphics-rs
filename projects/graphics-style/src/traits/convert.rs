use super::*;

impl Default for CircleWidth {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Default for LineWidth {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Default for PointSize {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Default for PolygonEdgeWidth {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Default for PolylineWidth {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Default for RectangleEdgeWidth {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Default for SquareEdgeWidth {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl Default for TriangleEdgeWidth {
    fn default() -> Self {
        Self { value: 1.0 }
    }
}

impl From<Color> for BackgroundColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for BackgroundColor {
    fn into(self) -> Color {
        self.value
    }
}

impl From<Color> for CircleColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for CircleColor {
    fn into(self) -> Color {
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

impl From<Color> for DiskEdgeColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for DiskEdgeColor {
    fn into(self) -> Color {
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

impl From<Color> for DiskFillColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for DiskFillColor {
    fn into(self) -> Color {
        self.value
    }
}

impl From<Color> for LineColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for LineColor {
    fn into(self) -> Color {
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

impl From<Color> for PointColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for PointColor {
    fn into(self) -> Color {
        self.value
    }
}

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

impl From<Color> for PolygonEdgeColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for PolygonEdgeColor {
    fn into(self) -> Color {
        self.value
    }
}

impl From<f32> for PolygonEdgeWidth {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl Into<f32> for PolygonEdgeWidth {
    fn into(self) -> f32 {
        self.value
    }
}

impl From<Color> for PolygonFillColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for PolygonFillColor {
    fn into(self) -> Color {
        self.value
    }
}

impl From<Color> for PolylineColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for PolylineColor {
    fn into(self) -> Color {
        self.value
    }
}

impl From<f32> for PolylineWidth {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl Into<f32> for PolylineWidth {
    fn into(self) -> f32 {
        self.value
    }
}

impl From<Color> for RectangleEdgeColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for RectangleEdgeColor {
    fn into(self) -> Color {
        self.value
    }
}

impl From<f32> for RectangleEdgeWidth {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl Into<f32> for RectangleEdgeWidth {
    fn into(self) -> f32 {
        self.value
    }
}

impl From<Color> for RectangleFillColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for RectangleFillColor {
    fn into(self) -> Color {
        self.value
    }
}

impl From<Color> for SquareEdgeColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for SquareEdgeColor {
    fn into(self) -> Color {
        self.value
    }
}

impl From<f32> for SquareEdgeWidth {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl Into<f32> for SquareEdgeWidth {
    fn into(self) -> f32 {
        self.value
    }
}

impl From<Color> for SquareFillColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for SquareFillColor {
    fn into(self) -> Color {
        self.value
    }
}

impl From<Color> for TextColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for TextColor {
    fn into(self) -> Color {
        self.value
    }
}

impl From<f32> for TextFont {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl Into<f32> for TextFont {
    fn into(self) -> f32 {
        self.value
    }
}

impl From<f32> for TextSize {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl Into<f32> for TextSize {
    fn into(self) -> f32 {
        self.value
    }
}

impl From<Color> for TriangleEdgeColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for TriangleEdgeColor {
    fn into(self) -> Color {
        self.value
    }
}

impl From<f32> for TriangleEdgeWidth {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl Into<f32> for TriangleEdgeWidth {
    fn into(self) -> f32 {
        self.value
    }
}

impl From<Color> for TriangleFillColor {
    fn from(value: Color) -> Self {
        Self { value }
    }
}

impl Into<Color> for TriangleFillColor {
    fn into(self) -> Color {
        self.value
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

impl PartialEq<f32> for PolygonEdgeWidth {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for PolygonEdgeWidth {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialEq<f32> for PolylineWidth {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for PolylineWidth {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialEq<f32> for RectangleEdgeWidth {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for RectangleEdgeWidth {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialEq<f32> for SquareEdgeWidth {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for SquareEdgeWidth {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialEq<f32> for TextFont {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for TextFont {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialEq<f32> for TextSize {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for TextSize {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}

impl PartialEq<f32> for TriangleEdgeWidth {
    fn eq(&self, other: &f32) -> bool {
        self.value.eq(other)
    }
}

impl PartialOrd<f32> for TriangleEdgeWidth {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.value.partial_cmp(other)
    }
}
