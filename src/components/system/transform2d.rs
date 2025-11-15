#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Transform2D {
    position: glam::Vec2,
    rotation: f32,
    scale: glam::Vec2,
}

impl Transform2D {
    pub fn identity() -> Self {
        Self {
            position: glam::Vec2::ZERO,
            rotation: 0.0,
            scale: glam::Vec2::ONE,
        }
    }
    pub fn set_position(&mut self, position: glam::Vec2) {
        self.position = position;
    }
    pub fn get_position(&self) -> glam::Vec2 {
        self.position
    }
    pub fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }
    pub fn get_rotation(&self) -> f32 {
        self.rotation
    }
    pub fn set_scale(&mut self, scale: glam::Vec2) {
        self.scale = scale;
    }
    pub fn get_scale(&self) -> glam::Vec2 {
        self.scale
    }

    pub fn matrix(&self) -> glam::Mat3 {
        let translation = glam::Mat3::from_translation(self.position);
        let rotation = glam::Mat3::from_rotation_z(self.rotation);
        let scale = glam::Mat3::from_scale(self.scale);
        translation * rotation * scale
    }
}
