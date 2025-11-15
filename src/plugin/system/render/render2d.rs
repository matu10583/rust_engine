use crate::components::Sprite;
use crate::components::Transform2D;
use crate::core::app::App;
use crate::core::plugin::Plugin;
use crate::core::schedule::{Stage, Priority};
use crate::events::system::{RenderCommand, RenderQueue};
pub struct Render2D {}

impl Render2D {
    pub fn new() -> Self {
        Self {}
    }
}

impl Plugin for Render2D {
    fn build(&self, app: &mut App) {
        app.add_event(
            RenderQueue::new(),
            Stage::PreRender,
            Priority::Normal,
        );
        app.add_system(
            Stage::LateUpdate,
            Priority::Low,
            collect_and_send_system,
        );
        app.add_system(
            Stage::Render,
            Priority::Normal,
            render_system,
        );
    }
}

fn render_system(di: &mut crate::core::DiContainer, _world: &mut crate::core::ecs::World) {
    if let Some(ev) = di.get_mut::<RenderQueue>() {
        let commands: Vec<RenderCommand> = ev.drain().collect();
        for command in commands {
            match command {
                RenderCommand::DrawSprite { sprite, transform } => {
                    // 実際のレンダリング処理はここに実装します。
                    println!(
                        "Rendering sprite {:?} at position ({}, {})",
                        sprite,
                        transform.get_position().x,
                        transform.get_position().y
                    );
                }
            }
        }
    }
}

fn collect_and_send_system(
    _di: &mut crate::core::DiContainer,
    _world: &mut crate::core::ecs::World,
) {
    // 2D Rendering logic goes here
    let mut cmds = Vec::new();
    collect_sprite(_world, &mut cmds);
    send_commands(_di, cmds);
}
fn collect_sprite(_world: &mut crate::core::ecs::World, _cmds: &mut Vec<RenderCommand>) {
    // スプライト収集ロジックをここに実装します。
    let world = _world;
    let mut targets = world.query_ref::<(Transform2D, Sprite)>();
    for (_e, (transform, sprite)) in targets.iter() {
        // エンティティごとに Transform と Sprite を使って描画コマンドを生成します。
        let cmd = RenderCommand::DrawSprite {
            sprite: sprite.clone(),
            transform: transform.clone(),
        };
        _cmds.push(cmd);
    }
}

fn send_commands(_di: &mut crate::core::DiContainer, _cmds: Vec<RenderCommand>) {
    if let Some(render_queue) = _di.get_mut::<RenderQueue>() {
        for cmd in _cmds {
            render_queue.send(cmd);
        }
    }
}
