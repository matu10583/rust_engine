use crate::core::app::App;
use crate::core::events::Events;
use crate::core::input::Input;
use crate::core::plugin::Plugin;
use crate::core::schedule::Stage;
use crate::core::DiContainer;
use crate::events::{CursorMovedEvent, KeyboardInputEvent, MouseInputEvent};

pub struct InputPlugin;

impl InputPlugin {
    pub fn new() -> Self {
        InputPlugin {}
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        fn input_system(di: &mut DiContainer, _world: &mut crate::core::ecs::World) {
            let input = di.get_mut::<Input>().unwrap();
            input.clear_frame();
            let _ = input;

            // Keyboard events
            if let Some(keyboard_events) = di.get_mut::<Events<KeyboardInputEvent>>() {
                let drained: Vec<_> = keyboard_events.drain().collect();
                let _ = keyboard_events;
                if let Some(input) = di.get_mut::<Input>() {
                    for event in drained {
                        if event.state == crate::core::input::EngineElementState::Pressed {
                            input.press_key(event.key);
                        } else {
                            input.release_key(event.key);
                        }
                    }
                }
            }

            // Mouse button events
            if let Some(mouse_events) = di.get_mut::<Events<MouseInputEvent>>() {
                let drained: Vec<_> = mouse_events.drain().collect();
                let _ = mouse_events;
                if let Some(input) = di.get_mut::<Input>() {
                    for event in drained {
                        if event.state == crate::core::input::EngineElementState::Pressed {
                            input.press_mouse_button(event.button);
                        } else {
                            input.release_mouse_button(event.button);
                        }
                    }
                }
            }

            // Cursor move events
            if let Some(cursor_events) = di.get_mut::<Events<CursorMovedEvent>>() {
                let drained: Vec<_> = cursor_events.drain().collect();
                let _ = cursor_events;
                if let Some(input) = di.get_mut::<Input>() {
                    for event in drained {
                        input.set_mouse_position(event.x, event.y);
                    }
                }
            }

            // 他のイベント処理は同様のパターンで追加
        }
        app.get_di_container().insert(Input::new());
        app.add_system(Stage::ProcessInput, input_system);
        app.add_event(crate::core::events::Events::<KeyboardInputEvent>::new());
        app.add_event(crate::core::events::Events::<MouseInputEvent>::new());
        app.add_event(crate::core::events::Events::<CursorMovedEvent>::new());
    }
}
