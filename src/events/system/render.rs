use crate::components::{Sprite, Transform2D};

pub enum RenderCommand {
    DrawSprite {
        sprite: Sprite,
        transform: Transform2D,
    },
}

pub type RenderQueue = crate::core::events::Events<RenderCommand>;
