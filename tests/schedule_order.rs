use rust_engine::core::ecs;
use rust_engine::core::schedule::{Priority, Schedule, Stage};
use rust_engine::core::DiContainer;

fn sys_a(di: &mut DiContainer, _world: &mut ecs::World) {
    if let Some(v) = di.get_mut::<Vec<i32>>() {
        v.push(10);
    }
}
fn sys_b(di: &mut DiContainer, _world: &mut ecs::World) {
    if let Some(v) = di.get_mut::<Vec<i32>>() {
        v.push(20);
    }
}
fn sys_c(di: &mut DiContainer, _world: &mut ecs::World) {
    if let Some(v) = di.get_mut::<Vec<i32>>() {
        v.push(30);
    }
}

#[test]
fn schedule_respects_priority_buckets() {
    let mut sched = Schedule::new();
    let mut di = DiContainer::new();
    di.insert(Vec::<i32>::new());
    let mut world = ecs::World::new();

    // Register systems in same stage but different priorities
    sched.add_system(Stage::Update, Priority::High, sys_a);
    sched.add_system(Stage::Update, Priority::Normal, sys_b);
    sched.add_system(Stage::Update, Priority::Low, sys_c);

    // Run the stage
    sched.run_stage(Stage::Update, &mut di, &mut world);

    let v = di.get_mut::<Vec<i32>>().unwrap();
    // Expect order: sys_a(10), sys_b(20), sys_c(30)
    assert_eq!(&*v, &[10, 20, 30]);
}
