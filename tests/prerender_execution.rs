use rust_engine::core::app::App;
use rust_engine::core::ecs;
use rust_engine::core::schedule::{Priority, Stage};
use rust_engine::core::DiContainer;

fn prerender_system(di: &mut DiContainer, _world: &mut ecs::World) {
    if let Some(v) = di.get_mut::<Vec<String>>() {
        v.push("PreRender".to_string());
    }
}

fn render_system(di: &mut DiContainer, _world: &mut ecs::World) {
    if let Some(v) = di.get_mut::<Vec<String>>() {
        v.push("Render".to_string());
    }
}

#[test]
fn prerender_runs_before_render_in_app_render_method() {
    let mut app = App::new();
    
    // Insert a Vec to track execution order
    app.get_di_container().insert(Vec::<String>::new());
    
    // Register systems in PreRender and Render stages
    app.add_system(Stage::PreRender, Priority::Normal, prerender_system);
    app.add_system(Stage::Render, Priority::Normal, render_system);
    
    // Call render which should execute PreRender then Render
    app.render(0.0);
    
    // Verify execution order
    let execution_order = app.get_di_container().get::<Vec<String>>().unwrap();
    assert_eq!(execution_order.len(), 2);
    assert_eq!(execution_order[0], "PreRender");
    assert_eq!(execution_order[1], "Render");
}
