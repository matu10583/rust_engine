use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, Default)]
pub struct Time {
    delta_seconds: Duration,
    elapsed_seconds: Duration,
}

impl Time {
    pub fn delta_seconds(&self) -> f32 {
        self.delta_seconds.as_secs_f32()
    }
    pub fn elapsed_seconds(&self) -> f32 {
        self.elapsed_seconds.as_secs_f32()
    }
}

pub struct TimeState {
    last_instant: Instant,
    time: Time,
}

impl TimeState {
    pub fn new() -> Self {
        Self {
            last_instant: Instant::now(),
            time: Time {
                delta_seconds: Duration::ZERO,
                elapsed_seconds: Duration::ZERO,
            },
        }
    }

    pub fn tick(&mut self) -> Time {
        let now = Instant::now();
        let delta = now - self.last_instant;
        self.last_instant = now;
        self.time.elapsed_seconds += delta;
        self.time.delta_seconds = delta;
        self.time.clone()
    }
}

pub struct TimeFixed {
    pub delta_seconds: f32,
}
impl TimeFixed {
    pub fn new(dt: f32) -> Self {
        Self { delta_seconds: dt }
    }
}
