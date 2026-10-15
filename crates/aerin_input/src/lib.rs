pub mod keyboard;
pub mod mouse;

pub use keyboard::KeyCode;
pub use mouse::MouseButton;

use std::collections::HashSet;

#[derive(Default, Debug, Clone)]
pub struct Input {
    keys_held: HashSet<KeyCode>,
    keys_pressed: HashSet<KeyCode>,
    keys_released: HashSet<KeyCode>,

    mouse_held: HashSet<MouseButton>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,

    mouse_delta: (f32, f32),
}

impl Input {
    pub fn new() -> Self {
        Self::default()
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

    pub fn is_mouse_held(&self, button: MouseButton) -> bool {
        self.mouse_held.contains(&button)
    }

    pub fn mouse_delta(&self) -> (f32, f32) {
        self.mouse_delta
    }

    pub fn press_key(&mut self, key: KeyCode) {
        if self.keys_held.insert(key) {
            self.keys_pressed.insert(key);
        }
    }

    pub fn release_key(&mut self, key: KeyCode) {
        self.keys_held.remove(&key);
        self.keys_released.insert(key);
    }

    pub fn press_mouse(&mut self, button: MouseButton) {
        if self.mouse_held.insert(button) {
            self.mouse_pressed.insert(button);
        }
    }

    pub fn release_mouse(&mut self, button: MouseButton) {
        self.mouse_held.remove(&button);
        self.mouse_released.insert(button);
    }

    pub fn set_mouse_delta(&mut self, dx: f32, dy: f32) {
        self.mouse_delta = (dx, dy);
    }

    pub fn clear_frame(&mut self) {
        self.keys_pressed.clear();
        self.keys_released.clear();
        self.mouse_pressed.clear();
        self.mouse_released.clear();
        self.mouse_delta = (0.0, 0.0);
    }
}
