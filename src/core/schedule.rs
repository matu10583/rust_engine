pub use crate::core::ecs;
pub use crate::core::{DiContainer, Events, Time, TimeState};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Stage {
    Startup,
    ProcessInput,
    Update,
    FixedUpdate,
    PreRender,
    Render,
    LateUpdate,
}

type System = fn(&mut DiContainer, &mut ecs::World);
/// Maximum allowed priority index. Values above this will be clamped to this value.
///
/// Use a named constant to avoid magic numbers sprinkled around the codebase.
pub const MAX_PRIORITY: usize = 8;
/// Convenience priority levels to avoid sprinkling raw numbers in callers.
///
/// These map to internal bucket indices; callers can pass `Priority` or a
/// `usize` directly (both are accepted by `add_system`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    Highest,
    High,
    Normal,
    Low,
    Lowest,
}

impl From<Priority> for usize {
    fn from(p: Priority) -> usize {
        match p {
            Priority::Highest => 0,
            Priority::High => 1,
            Priority::Normal => 4,
            Priority::Low => 6,
            Priority::Lowest => MAX_PRIORITY,
        }
    }
}
pub struct Schedule {
    // Each stage holds priority buckets (Vec of Vec<System>). Lower index = higher priority (runs earlier).
    startup: Vec<Vec<System>>,
    update: Vec<Vec<System>>,
    late_update: Vec<Vec<System>>,
    render: Vec<Vec<System>>,
    fixed_update: Vec<Vec<System>>,
    process_input: Vec<Vec<System>>,
    pre_render: Vec<Vec<System>>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            // start each stage with a single default priority bucket
            startup: vec![vec![]],
            update: vec![vec![]],
            late_update: vec![vec![]],
            render: vec![vec![]],
            fixed_update: vec![vec![]],
            process_input: vec![vec![]],
            pre_render: vec![vec![]],
        }
    }
    // 注意: スケジュール API は優先度優先です。システム登録時に明示的な
    // 優先度を渡してください。`add_system` は優先度を受け取り、使用を簡潔
    // にして自己説明的にします。

    /// 指定したステージにシステムを追加します（優先度バケット付き）。
    /// `priority` の値が小さいほど先に実行されます。バケットは必要に応じて作成されます。
    ///
    /// 補足: 内部バケットの無制限拡張を防ぐため、`priority` は `MAX_PRIORITY` にクランプ
    /// されます。`MAX_PRIORITY` を超える値が与えられた場合はログで警告し、クランプします。
    pub fn add_system<I: Into<usize>>(
        &mut self,
        stage: Stage,
        priority: I,
        system: System,
    ) -> &mut Self {
        let v = match stage {
            Stage::Startup => &mut self.startup,
            Stage::Update => &mut self.update,
            Stage::LateUpdate => &mut self.late_update,
            Stage::Render => &mut self.render,
            Stage::FixedUpdate => &mut self.fixed_update,
            Stage::ProcessInput => &mut self.process_input,
            Stage::PreRender => &mut self.pre_render,
        };
        // Accept either a `usize` or a `Priority` (which implements Into<usize>).
        let priority = priority.into();

        // Clamp the incoming priority to a sane upper bound to prevent a
        // caller from accidentally creating a huge number of empty buckets.
        let capped = if priority > MAX_PRIORITY {
            log::warn!(
                "Schedule::add_system: priority {} > MAX_PRIORITY ({}); clamping to {}",
                priority,
                MAX_PRIORITY,
                MAX_PRIORITY
            );
            MAX_PRIORITY
        } else {
            priority
        };

        // Ensure enough buckets
        while v.len() <= capped {
            v.push(vec![]);
        }
        v[capped].push(system);
        self
    }
    pub fn run_stage(&mut self, stage: Stage, di: &mut DiContainer, world: &mut ecs::World) {
        let buckets = match stage {
            Stage::Startup => &mut self.startup,
            Stage::Update => &mut self.update,
            Stage::LateUpdate => &mut self.late_update,
            Stage::Render => &mut self.render,
            Stage::FixedUpdate => &mut self.fixed_update,
            Stage::ProcessInput => &mut self.process_input,
            Stage::PreRender => &mut self.pre_render,
        };

        // Iterate buckets in order; within each bucket preserve insertion order.
        for bucket in buckets.iter_mut() {
            for system in bucket.iter().copied() {
                system(di, world);
            }
        }
    }
}

// Generic flush system for Events<T>.
// Register with: app.add_system(Stage::LateUpdate, Priority::Normal, crate::core::schedule::flush_events::<YourEvent>);
pub fn flush_events<T: 'static + Send + Sync>(di: &mut DiContainer, _world: &mut ecs::World) {
    if let Some(events) = di.get_mut::<Events<T>>() {
        events.update();
    }
}
