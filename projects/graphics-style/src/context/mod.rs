mod resolver;

use crate::{context::resolver::GraphicsStyle::PointColor, PointColor, PointSize, PointStyle, RGBA};
pub(crate) use resolver::StyleContext;

/// Resolve missing style
#[derive(Debug, Clone, Default)]
pub struct StyleResolver {
    theme: StyleContext,
    local: StyleContext,
}

#[derive(Debug, Clone, Default)]
pub struct StyleContext {
    pub point_size: Option<f32>,
    pub point_color: Option<RGBA>,
    pub line_color: Option<RGBA>,
    pub line_width: Option<f32>,
}
