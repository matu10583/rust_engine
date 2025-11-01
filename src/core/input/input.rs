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
        if (self.is_key_down(key)) {
            return;
        }
        self.keys_pressed.insert(key);
        self.keys_down.insert(key);
    }
    pub fn release_key(&mut self, key: EngineKey) {
        self.keys_down.remove(&key);
    }

    pub fn press_mouse_button(&mut self, button: EngineMouseButton) {
        if (self.is_mouse_button_down(button)) {
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
