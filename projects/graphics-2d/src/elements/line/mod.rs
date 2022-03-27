mod traits;
use super::*;

impl Line {
    pub fn from_2_points(start: &Point, end: &Point) -> Self {
        Self { x1: start.x, y1: start.y, x2: end.x, y2: end.y, ..Default::default() }
    }
}

impl Line {
    pub fn is_empty(&self, ctx: &StyleResolver) -> bool {
        false
        // let length = || self.start == self.end;
        // let width = || self.width.unwrap_or(ctx.line_width()) <= 0.0;
        // length() || width()
    }
}
