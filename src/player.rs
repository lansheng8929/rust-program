use winit::keyboard::KeyCode;

use crate::{input_state::InputState, uitils::constrain_to_bounds};

pub(crate) struct Player {
    width: i16,
    height: i16,
    x: i16,
    y: i16,
    speed: i16,
    input_state: InputState,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            width: 10,
            height: 10,
            x: 10,
            y: 10,
            speed: 2,
            input_state: InputState::default(),
        }
    }
}

impl Player {
    pub(crate) fn new(size: i16, x: i16, y: i16, speed: i16) -> Self {
        Self {
            width: size,
            height: size,
            x,
            y,
            speed,
            input_state: InputState::default(),
        }
    }

    pub(crate) fn update(&mut self, width: u32, height: u32) {
        self.handle_input();

        let (x_constrained, y_constrained) = constrain_to_bounds(
            self.x as i32,
            self.y as i32,
            width.saturating_sub(self.width as u32),
            height.saturating_sub(self.height as u32),
        );
        self.x = x_constrained as i16;
        self.y = y_constrained as i16;
    }

    pub(crate) fn draw(&self, x: i16, y: i16) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    pub(crate) fn input(&mut self, key_code: KeyCode, pressed: bool) {
        self.input_state.handle_key_state(key_code, pressed);
    }

    fn handle_input(&mut self) {
        if self.input_state.left_pressed {
            self.x -= self.speed;
        }
        if self.input_state.right_pressed {
            self.x += self.speed;
        }
        if self.input_state.up_pressed {
            self.y -= self.speed;
        }
        if self.input_state.down_pressed {
            self.y += self.speed;
        }
    }
}
