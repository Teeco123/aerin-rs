use std::collections::HashSet;

use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{
        KeyCode::{self},
        PhysicalKey,
    },
};

#[derive(Default)]
pub struct InputManager {
    keys_held: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,

    cursor_delta: (f64, f64),
}
impl InputManager {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn clear_input(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
    }

    pub fn is_key_held(&self, key: KeyCode) -> bool {
        self.keys_held.contains(&key)
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn is_key_released(&self, key: KeyCode) -> bool {
        self.keys_released.contains(&key)
    }
}

impl InputManager {
    pub(crate) fn process_keyboard(&mut self, event: KeyEvent) {
        if let PhysicalKey::Code(code) = event.physical_key {
            match event.state {
                ElementState::Pressed => {
                    if self.keys_held.insert(code) {
                        self.keys_pressed.insert(code);
                    }
                }
                ElementState::Released => {
                    self.keys_held.remove(&code);
                    self.keys_released.insert(code);
                }
            }
        }
    }

    pub(crate) fn process_mouse_movement(&mut self, delta: (f64, f64)) {
        self.cursor_delta = delta;
    }
}
