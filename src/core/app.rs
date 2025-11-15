use crate::core::ecs;
use crate::core::plugin::Plugin;
use crate::core::schedule::{Schedule, Stage};
use crate::core::{DiContainer, Time, TimeFixed, TimeState};

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
        dicontainer.insert(TimeFixed::new(1.0 / 60.0)); // 固定更新用の時間間隔を追加
        Self {
            dicontainer: dicontainer,
            world: ecs::World::new(),
            timer_state: TimeState::new(),
            schedule: Schedule::new(),
            run_startup: false,
        }
    }

    pub fn set_fixed_dt(&mut self, dt: f32) {
        if let Some(fixed_time) = self.dicontainer.get_mut::<TimeFixed>() {
            fixed_time.delta_seconds = dt;
        }
    }
    /// 指定した優先度でスケジュールにシステムを登録します。
    pub fn add_system<I: Into<usize>>(
        &mut self,
        stage: Stage,
        priority: I,
        system: fn(&mut DiContainer, &mut ecs::World),
    ) -> &mut Self {
        self.schedule.add_system(stage, priority, system);
        self
    }

    pub fn get_di_container(&mut self) -> &mut DiContainer {
        &mut self.dicontainer
    }

    pub fn get_world(&mut self) -> &mut ecs::World {
        &mut self.world
    }

    pub fn add_plugin<P: Plugin>(&mut self, plugin: &P) -> &mut Self {
        plugin.build(self);
        self
    }

    /// Register an Events<T> resource and add an `update()` system into the
    /// given `update_stage` at the default priority (0).
    /// `Events<T>` リソースを登録し、その `update()` を指定した `update_stage` と
    /// 優先度で実行するシステムとして登録します。優先度インデックスが小さいほど先に実行されます。
    pub fn add_event<T: 'static + Send + Sync, I: Into<usize>>(
        &mut self,
        event: crate::core::events::Events<T>,
        update_stage: Stage,
        priority: I,
    ) -> &mut Self {
        fn update_event<T: 'static + Send + Sync>(di: &mut DiContainer, _world: &mut ecs::World) {
            if let Some(events) = di.get_mut::<crate::core::events::Events<T>>() {
                events.update();
            }
        }
        self.dicontainer.insert(event);
        self.schedule
            .add_system(update_stage, priority, update_event::<T>);
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

    pub fn tick_timer(&mut self) {
        let t = self.timer_state.tick();

        if let Some(time) = self.dicontainer.get_mut::<Time>() {
            *time = t;
        }
    }

    pub fn process_input(&mut self) {
        self.schedule
            .run_stage(Stage::ProcessInput, &mut self.dicontainer, &mut self.world);
    }

    pub fn update_logic(&mut self) {
        self.schedule
            .run_stage(Stage::Update, &mut self.dicontainer, &mut self.world);
    }

    pub fn render(&mut self, _alpha: f32) {
        // レンダリングロジック（必要に応じて実装）
        self.schedule
            .run_stage(Stage::Render, &mut self.dicontainer, &mut self.world);
    }

    pub fn late_update(&mut self) {
        self.schedule
            .run_stage(Stage::LateUpdate, &mut self.dicontainer, &mut self.world);
    }

    pub fn fixed_update(&mut self) {
        // 固定更新ロジック（必要に応じて実装）
        self.schedule
            .run_stage(Stage::FixedUpdate, &mut self.dicontainer, &mut self.world);
    }
}
