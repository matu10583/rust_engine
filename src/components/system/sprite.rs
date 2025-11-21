use crate::core::TextureHandle;

#[derive(Debug, Clone)]
pub struct SpriteHandle {
    id: u32,
}
#[derive(Debug, Clone)]
pub struct Sprite {
    handle: TextureHandle,
    tint: [f32; 4],
    pivot: glam::Vec2,
    visible: bool,
}

impl Sprite {
    pub fn new(handle: TextureHandle) -> Self {
        Self {
            handle,
            tint: [1.0, 1.0, 1.0, 1.0],
            pivot: glam::Vec2::new(0.5, 0.5),
            visible: true,
        }
    }

    /// このスプライトが保持するハンドルへの参照を返します。
    pub fn handle(&self) -> &TextureHandle {
        &self.handle
    }

    pub fn set_tint(&mut self, tint: [f32; 4]) {
        self.tint = tint;
    }

    pub fn get_tint(&self) -> [f32; 4] {
        self.tint
    }

    pub fn set_pivot(&mut self, pivot: glam::Vec2) {
        self.pivot = pivot;
    }

    pub fn get_pivot(&self) -> glam::Vec2 {
        self.pivot
    }

    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    pub fn is_visible(&self) -> bool {
        self.visible
    }
}
