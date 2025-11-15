use crate::core::input::types::{EngineKey, EngineMouseButton};
use std::collections::HashSet;
pub struct Input {
    pub keys_pressed: HashSet<EngineKey>,
    pub keys_down: HashSet<EngineKey>,
    pub mouse_button_pressed: HashSet<EngineMouseButton>,
    pub mouse_button_down: HashSet<EngineMouseButton>,
    pub mouse_position: (f32, f32),
}

impl Input {
    pub fn new() -> Self {
        Self {
            keys_pressed: HashSet::new(),
            keys_down: HashSet::new(),
            mouse_button_pressed: HashSet::new(),
            mouse_button_down: HashSet::new(),
            mouse_position: (0.0, 0.0),
        }
    }

    pub fn is_key_pressed(&self, key: EngineKey) -> bool {
        self.keys_pressed.contains(&key)
    }
    pub fn is_key_down(&self, key: EngineKey) -> bool {
        self.keys_down.contains(&key)
    }

    pub fn is_mouse_button_pressed(&self, button: EngineMouseButton) -> bool {
        self.mouse_button_pressed.contains(&button)
    }
    pub fn is_mouse_button_down(&self, button: EngineMouseButton) -> bool {
        self.mouse_button_down.contains(&button)
    }

    pub fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = (x, y);
    }
    pub fn get_mouse_position(&self) -> (f32, f32) {
        self.mouse_position
    }

    pub fn press_key(&mut self, key: EngineKey) {
        if self.is_key_down(key) {
            return;
        }
        self.keys_pressed.insert(key);
        self.keys_down.insert(key);
    }
    pub fn release_key(&mut self, key: EngineKey) {
        self.keys_down.remove(&key);
    }

    pub fn press_mouse_button(&mut self, button: EngineMouseButton) {
        if self.is_mouse_button_down(button) {
            return;
        }
        self.mouse_button_pressed.insert(button);
        self.mouse_button_down.insert(button);
    }

    pub fn release_mouse_button(&mut self, button: EngineMouseButton) {
        self.mouse_button_down.remove(&button);
    }

    pub fn clear_frame(&mut self) {
        self.keys_pressed.clear();
        self.mouse_button_pressed.clear();
    }

    pub fn lost_focus(&mut self) {
        self.keys_down.clear();
        self.keys_pressed.clear();
        self.mouse_button_down.clear();
        self.mouse_button_pressed.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::events::Events;
    use crate::core::input::EngineElementState;
    use crate::events::KeyboardInputEvent;

    #[test]
    fn input_press_release_and_clear() {
        let mut input = Input::new();
        assert!(!input.is_key_down(EngineKey::A));
        assert!(!input.is_key_pressed(EngineKey::A));

        input.press_key(EngineKey::A);
        assert!(input.is_key_down(EngineKey::A));
        assert!(input.is_key_pressed(EngineKey::A));

        // clear_frame should clear the 'pressed' set but keep 'down'
        input.clear_frame();
        assert!(input.is_key_down(EngineKey::A));
        assert!(!input.is_key_pressed(EngineKey::A));

        // release should remove from 'down'
        input.release_key(EngineKey::A);
        assert!(!input.is_key_down(EngineKey::A));
    }

    #[test]
    fn events_double_buffer_and_apply_to_input_like_plugin() {
        // Events queue behavior: sends are visible only after update()
        let mut evts: Events<KeyboardInputEvent> = Events::new();
        let evt = KeyboardInputEvent {
            key: EngineKey::A,
            state: EngineElementState::Pressed,
        };

        evts.send(evt);
        // Not yet visible on the read side
        assert!(evts.is_empty());

        // Flip buffers so sent events become readable
        evts.update();
        assert_eq!(evts.len(), 1);

        // Drain and apply to an Input instance (mimic InputPlugin behavior)
        let mut input = Input::new();
        for e in evts.drain() {
            if e.state == EngineElementState::Pressed {
                input.press_key(e.key);
            } else {
                input.release_key(e.key);
            }
        }

        assert!(input.is_key_down(EngineKey::A));
        assert!(input.is_key_pressed(EngineKey::A));

        // After draining, the buffer should be empty
        assert_eq!(evts.len(), 0);
    }
}
