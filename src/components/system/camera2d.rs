use crate::components::Transform2D;

#[derive(Debug, Clone)]
pub struct Camera2D {
    transform: Transform2D,
    zoom: f32,
    viewport: (f32, f32),
}

impl Camera2D {
    pub fn new(viewport_width: f32, viewport_height: f32) -> Self {
        Self {
            transform: Transform2D::identity(),
            zoom: 1.0,
            viewport: (viewport_width, viewport_height),
        }
    }

    pub fn set_transform(&mut self, transform: Transform2D) {
        self.transform = transform;
    }
    pub fn get_transform(&self) -> Transform2D {
        self.transform
    }

    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom;
    }
    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn set_viewport(&mut self, width: f32, height: f32) {
        self.viewport = (width, height);
    }
    pub fn get_viewport(&self) -> (f32, f32) {
        self.viewport
    }

    pub fn view_matrix(&self) -> glam::Mat3 {
        let translation = glam::Mat3::from_translation(-self.transform.get_position());
        let rotation = glam::Mat3::from_rotation_z(-self.get_transform().get_rotation());
        let scale = glam::Mat3::from_scale(glam::Vec2::splat(1.0 / self.zoom));
        scale * rotation * translation
    }

    pub fn proj_matrix(&self) -> glam::Mat3 {
        let (width, height) = self.viewport;
        glam::Mat3::from_scale(glam::Vec2::new(2.0 / width, -2.0 / height))
    }

    pub fn world_to_screen(&self, world_pos: glam::Vec2) -> glam::Vec2 {
        let clip_pos = self.proj_matrix() * self.view_matrix() * world_pos.extend(1.0);
        glam::Vec2::new(
            (clip_pos.x + 1.0) * 0.5 * self.viewport.0,
            (1.0 - (clip_pos.y + 1.0) * 0.5) * self.viewport.1,
        )
    }

    pub fn screen_to_world(&self, screen_pos: glam::Vec2) -> glam::Vec2 {
        let clip_x = (screen_pos.x / self.viewport.0) * 2.0 - 1.0;
        let clip_y = 1.0 - (screen_pos.y / self.viewport.1) * 2.0;
        let clip_pos = glam::Vec3::new(clip_x, clip_y, 1.0);

        let inv_proj = self.proj_matrix().inverse();
        let inv_view = self.view_matrix().inverse();
        let world_pos = inv_view * inv_proj * clip_pos;
        world_pos.truncate()
    }
}
