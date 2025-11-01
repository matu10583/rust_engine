pub use crate::core::input::{EngineElementState, EngineKey, EngineMouseButton};
pub struct KeyboardInputEvent {
    pub key: EngineKey,
    pub state: EngineElementState,
}

pub struct MouseInputEvent {
    pub button: EngineMouseButton,
    pub state: EngineElementState,
}

pub struct CursorMovedEvent {
    pub x: f32,
    pub y: f32,
}
