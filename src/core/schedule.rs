pub use crate::core::ecs;
pub use crate::core::{DiContainer, Events, Time, TimeState};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Stage {
    Startup,
    ProcessInput,
    Update,
    LateUpdate,
    Render,
    FixedUpdate,
}

type System = fn(&mut DiContainer, &mut ecs::World);
pub struct Schedule {
    startup: Vec<System>,
    update: Vec<System>,
    late_update: Vec<System>,
    render: Vec<System>,
    fixed_update: Vec<System>,
    process_input: Vec<System>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            startup: vec![],
            update: vec![],
            late_update: vec![],
            render: vec![],
            fixed_update: vec![],
            process_input: vec![],
        }
    }

    pub fn add_system(&mut self, stage: Stage, system: System) -> &mut Self {
        let v = match stage {
            Stage::Startup => &mut self.startup,
            Stage::Update => &mut self.update,
            Stage::LateUpdate => &mut self.late_update,
            Stage::Render => &mut self.render,
            Stage::FixedUpdate => &mut self.fixed_update,
            Stage::ProcessInput => &mut self.process_input,
        };
        v.push(system);
        self
    }

    pub fn run_stage(&mut self, stage: Stage, di: &mut DiContainer, world: &mut ecs::World) {
        let v = match stage {
            Stage::Startup => &mut self.startup,
            Stage::Update => &mut self.update,
            Stage::LateUpdate => &mut self.late_update,
            Stage::Render => &mut self.render,
            Stage::FixedUpdate => &mut self.fixed_update,
            Stage::ProcessInput => &mut self.process_input,
        };
        for system in v.iter().copied() {
            system(di, world);
        }
    }
}

// Generic flush system for Events<T>.
// Register with: app.add_system(Stage::LateUpdate, crate::core::schedule::flush_events::<YourEvent>);
pub fn flush_events<T: 'static + Send + Sync>(di: &mut DiContainer, _world: &mut ecs::World) {
    if let Some(events) = di.get_mut::<Events<T>>() {
        events.update();
    }
}
