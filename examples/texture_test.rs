use rust_engine::core;
use rust_engine::core::TextureManager;
use rust_engine::platform::PollResult;
use rust_engine::platform::WinitBackend;
use rust_engine::{InputPlugin, Sprite, Transform2D};

fn main() {
    let mut app = core::app::App::new();
    app.startup();
    app.set_fixed_dt(1.0 / 60.0);
    app.add_plugin(&InputPlugin::new());

    // TextureManager をセットアップ
    {
        let di = app.get_di_container();
        di.insert(TextureManager::new());
    }

    // テクスチャをロードしてスプライトエンティティを作成
    setup_sprites(&mut app);

    let mut winit_backend = WinitBackend::new();
    while winit_backend.poll_once(&mut app) != PollResult::Exit {}
}

fn setup_sprites(app: &mut core::app::App) {
    let texture_handle = {
        let di = app.get_di_container();
        let mgr = di
            .get_mut::<TextureManager>()
            .expect("TextureManager not found");
        mgr.load("assets/player.png")
    };

    let world = app.get_world();

    // スプライトエンティティを作成
    let mut transform = Transform2D::identity();
    transform.set_position(glam::Vec2::new(100.0, 100.0));

    let sprite = Sprite::new(texture_handle);

    world.spawn((transform, sprite));

    println!(
        "Sprite entity created with texture handle: {:?}",
        texture_handle
    );
}
