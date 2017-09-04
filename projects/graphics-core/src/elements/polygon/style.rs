use super::*;

impl Triangle {
    /// Get the 3 vertex of a triangle
    pub fn get_vertex(&self) -> &[(f32, f32); 3] {
        &self.vertex
    }
}

impl Parallelogram {
    /// Get the 4 vertex of a parallelogram
    pub fn get_vertex(&self) -> &[(f32, f32); 4] {
        &self.vertex
    }
}

impl Polygon {
    /// Get the n vertex of a parallelogram
    pub fn get_vertex(&self) -> &Vec<(f32, f32)> {
        &self.vertex
    }
}
