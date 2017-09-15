use super::*;

impl Square {
    /// Get the x-coordinate of the left top point
    pub fn get_x(&self) -> &f32 {
        &self.x
    }

    /// Get the y-coordinate of the left top point
    pub fn get_y(&self) -> &f32 {
        &self.y
    }

    /// Get the side length of square
    pub fn get_side(&self) -> &f32 {
        &self.side
    }
}

impl Rectangle {
    /// Get the x-coordinate of the left top point
    pub fn get_x(&self) -> &f32 {
        &self.x
    }

    /// Get the y-coordinate of the left top point
    pub fn get_y(&self) -> &f32 {
        &self.y
    }

    /// Get the side length of square
    pub fn get_width(&self) -> &f32 {
        &self.width
    }

    /// Get the side length of square
    pub fn get_height(&self) -> &f32 {
        &self.height
    }
}
