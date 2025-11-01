use crate::core::events::Events;
use crate::core::input::{EngineKey, EngineMouseButton};
use crate::core::App;
use crate::events::{CursorMovedEvent, KeyboardInputEvent, MouseInputEvent};
use std::time::{Duration, Instant};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::platform::run_return::EventLoopExtRunReturn;
use winit::window::WindowBuilder;

pub struct WinitBackend {
    event_loop: EventLoop<()>,
    window: winit::window::Window,
    accumulator: Duration,
    last_instant: Instant,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PollResult {
    Exit,
    Continue,
}
impl WinitBackend {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title("Rust Engine")
            .build(&event_loop)
            .unwrap();

        WinitBackend {
            event_loop,
            window,
            accumulator: Duration::ZERO,
            last_instant: Instant::now(),
        }
    }

    pub fn poll_once(&mut self, app: &mut App) -> PollResult {
        let fixed_dt =
            if let Some(tf) = app.get_di_container().get::<crate::core::time::TimeFixed>() {
                Duration::from_secs_f32(tf.delta_seconds)
            } else {
                Duration::from_secs_f32(1.0 / 60.0)
            };

        let mut should_exit = false;
        let mut accumulator = self.accumulator;
        let mut last_instant = self.last_instant;

        self.event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Poll; // wait for next events by default
            match event {
                winit::event::Event::MainEventsCleared => {
                    app.process_input();

                    let now = Instant::now();
                    let mut frame_time = now - last_instant;
                    last_instant = now;

                    if frame_time > Duration::from_secs_f32(0.25) {
                        frame_time = Duration::from_secs_f32(0.25); // safety clamp
                    }
                    accumulator += frame_time;
                    app.tick_timer();
                    //可変Update
                    app.update_logic();
                    //固定Update
                    while accumulator >= fixed_dt {
                        app.fixed_update();
                        accumulator -= fixed_dt;
                    }
                    //Render
                    let alpha = accumulator.as_secs_f32() / fixed_dt.as_secs_f32();
                    app.render(alpha);

                    app.late_update();

                    // ウィンドウの再描画要求
                    self.window.request_redraw();
                    *control_flow = ControlFlow::Exit; // Exit after one frame
                }
                winit::event::Event::RedrawRequested(_) => {
                    // 描画処理
                }
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::KeyboardInput { input, .. } => {
                        // キーボード入力処理
                        if let Some(ev_queue) = app
                            .get_di_container()
                            .get_mut::<Events<KeyboardInputEvent>>()
                        {
                            ev_queue.send(KeyboardInputEvent {
                                key: EngineKey::from(input.virtual_keycode.unwrap()),
                                state: input.state.into(),
                            });
                        }
                    }
                    winit::event::WindowEvent::MouseInput { button, state, .. } => {
                        // マウス入力処理
                        if let Some(ev_queue) =
                            app.get_di_container().get_mut::<Events<MouseInputEvent>>()
                        {
                            ev_queue.send(MouseInputEvent {
                                button: EngineMouseButton::from(button),
                                state: state.into(),
                            });
                        }
                    }
                    winit::event::WindowEvent::CursorMoved { position, .. } => {
                        // マウス移動処理
                        if let Some(ev_queue) =
                            app.get_di_container().get_mut::<Events<CursorMovedEvent>>()
                        {
                            ev_queue.send(CursorMovedEvent {
                                x: position.x as f32,
                                y: position.y as f32,
                            });
                        }
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                        should_exit = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        });

        // Update self with the final state
        self.accumulator = accumulator;
        self.last_instant = last_instant;

        if should_exit {
            PollResult::Exit
        } else {
            PollResult::Continue
        }
    }
}
