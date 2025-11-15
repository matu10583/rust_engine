use rust_engine::core;
use rust_engine::platform::PollResult;
use rust_engine::platform::WinitBackend;
use rust_engine::InputPlugin;

fn main() {
    let mut app = core::app::App::new();
    app.startup();
    app.set_fixed_dt(1.0 / 60.0);
    app.add_plugin(&InputPlugin::new());

    let mut winit_backend = WinitBackend::new();
    while winit_backend.poll_once(&mut app) != PollResult::Exit {}
}
