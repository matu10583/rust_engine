pub mod core;
pub mod platform;
pub use platform::WinitBackend;
pub mod events;
pub mod plugin;
pub use plugin::InputPlugin;
pub mod components;
pub use components::{Camera2D, Sprite, Transform2D};
