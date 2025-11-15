use crate::core::app::App;
use crate::core::plugin::Plugin;
use crate::core::schedule::Stage;

#[allow(dead_code)]
trait Renderer {
    fn start_frame(&mut self, app: &mut App);
    fn end_frame(&mut self, app: &mut App);
}
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
        app.add_system(Stage::Render, render_system);
    }
}
impl Renderer for NullRenderer {
    fn start_frame(&mut self, _app: &mut App) {
        // No-op
    }

    fn end_frame(&mut self, _app: &mut App) {
        // No-op
    }
}
