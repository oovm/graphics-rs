mod resolver;

pub use resolver::GraphicsStyle;
pub(crate) use resolver::StyleContext;

/// Resolve missing style
#[derive(Debug, Clone, Default)]
pub struct StyleResolver {
    theme: StyleContext,
    local: StyleContext,
}
