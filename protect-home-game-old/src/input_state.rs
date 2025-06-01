use winit::keyboard::KeyCode;

pub struct InputState {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub shoot_pressed: bool,
}

impl Default for InputState {
    fn default() -> Self {
        Self {
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            shoot_pressed: false,
        }
    }
}

impl InputState {
    pub fn handle_key_state(&mut self, key_code: KeyCode, pressed: bool) {
        match key_code {
            KeyCode::KeyW => self.up_pressed = pressed,
            KeyCode::KeyA => self.left_pressed = pressed,
            KeyCode::KeyS => self.down_pressed = pressed,
            KeyCode::KeyD => self.right_pressed = pressed,
            KeyCode::Space => self.shoot_pressed = pressed,
            _ => (),
        }
    }
}
