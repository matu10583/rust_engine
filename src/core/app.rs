use crate::core::plugin::Plugin;
use crate::core::schedule::{Schedule, Stage};
use crate::ecs;
use crate::{DiContainer, Time, TimeState};

pub struct App {
    // App implementation
    dicontainer: DiContainer,
    world: ecs::World,
    timer_state: TimeState,
    schedule: Schedule,
    run_startup: bool,
}

impl App {
    pub fn new() -> Self {
        let mut dicontainer = DiContainer::new();
        dicontainer.insert(Time::default());
        Self {
            dicontainer: dicontainer,
            world: ecs::World::new(),
            timer_state: TimeState::new(),
            schedule: Schedule::new(),
            run_startup: false,
        }
    }

    pub fn add_system(
        &mut self,
        stage: Stage,
        system: fn(&mut DiContainer, &mut ecs::World),
    ) -> &mut Self {
        self.schedule.add_system(stage, system);
        self
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: &P) -> &mut Self {
        plugin.build(self);
        self
    }

    pub fn startup(&mut self) {
        if self.run_startup {
            return;
        }
        self.schedule
            .run_stage(Stage::Startup, &mut self.dicontainer, &mut self.world);
        self.run_startup = true;
    }

    pub fn update(&mut self) {
        let t = self.timer_state.tick();

        if let Some(time) = self.dicontainer.get_mut::<Time>() {
            *time = t;
        }

        self.schedule
            .run_stage(Stage::Update, &mut self.dicontainer, &mut self.world);
        self.schedule
            .run_stage(Stage::Render, &mut self.dicontainer, &mut self.world);
        self.schedule
            .run_stage(Stage::LateUpdate, &mut self.dicontainer, &mut self.world);
    }
}
