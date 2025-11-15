use crate::core::app::App;
use crate::core::plugin::Plugin;
use crate::core::schedule::Stage;

pub struct NullRenderer;

impl NullRenderer {
    pub fn new() -> Self {
        Self
    }
}

impl Plugin for NullRenderer {
    fn build(&self, app: &mut App) {
        fn render_system(_di: &mut crate::core::DiContainer, _world: &mut crate::core::ecs::World) {
            // Rendering logic goes here
        }
        app.add_system(
            Stage::Render,
            crate::core::schedule::Priority::Normal,
            render_system,
        );
    }
}
