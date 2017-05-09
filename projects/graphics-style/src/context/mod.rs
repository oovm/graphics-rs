use super::*;

#[derive(Debug, Clone)]
pub(crate) struct StyleContext {
    pub point_size: Option<PointSize>,
}

impl StyleResolver {
    /// Set the value of [`PointSize`]
    pub fn point_size(&self) -> PointSize {
        self.local.point_size.unwrap_or(self.theme.point_size.unwrap_or(PointSize::default()))
    }
}
