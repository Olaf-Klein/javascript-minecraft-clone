use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct InputState {
    keys_pressed: HashSet<KeyCode>,
    mouse_delta: (f64, f64),
    mouse_captured: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn key_pressed(&mut self, key: KeyCode) {
        self.keys_pressed.insert(key);
    }

    pub fn key_released(&mut self, key: KeyCode) {
        self.keys_pressed.remove(&key);
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&key)
    }

    pub fn update_mouse_delta(&mut self, delta: (f64, f64)) {
        // Accumulate mouse delta per-frame (device events may arrive multiple times)
        self.mouse_delta.0 += delta.0;
        self.mouse_delta.1 += delta.1;
    }

    pub fn get_mouse_delta(&self) -> (f64, f64) {
        self.mouse_delta
    }

    pub fn reset_mouse_delta(&mut self) {
        self.mouse_delta = (0.0, 0.0);
    }

    pub fn set_mouse_captured(&mut self, captured: bool) {
        self.mouse_captured = captured;
    }

    pub fn is_mouse_captured(&self) -> bool {
        self.mouse_captured
    }

    pub fn is_forward(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyW)
    }

    pub fn is_backward(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyS)
    }

    pub fn is_left(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyA)
    }

    pub fn is_right(&self) -> bool {
        self.is_key_pressed(KeyCode::KeyD)
    }

    pub fn is_up(&self) -> bool {
        self.is_key_pressed(KeyCode::Space)
    }

    pub fn is_down(&self) -> bool {
        self.is_key_pressed(KeyCode::ShiftLeft) || self.is_key_pressed(KeyCode::ShiftRight)
    }
}
