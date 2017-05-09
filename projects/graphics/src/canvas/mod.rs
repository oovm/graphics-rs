use crate::Graphic;

#[derive(Debug, Clone)]
pub struct Graphics {
    pub graphic: Vec<Graphic>,
    pub context: GraphicsContext,
}

#[derive(Debug, Clone)]
pub struct GraphicsContext {
    pub point_size: f32,
}
