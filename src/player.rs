use winit::keyboard::KeyCode;

use crate::{input_state::InputState, rectangle::Rectangle, uitils::constrain_to_bounds};

pub struct Player {
    pub bounds: Rectangle,
    speed: i32,
    input_state: InputState,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            bounds: Rectangle::new(10, 10, 10, 10),
            speed: 2,
            input_state: InputState::default(),
        }
    }
}

impl Player {
    pub fn new(size: u32, x: i32, y: i32, speed: i32) -> Self {
        let mut bounds = Rectangle::new(x, y, size, size);
        bounds.load_texture("player.png");
        Self {
            bounds,
            speed,
            input_state: InputState::default(),
        }
    }

    pub fn update(&mut self, width: u32, height: u32) {
        self.handle_input();

        let (x_constrained, y_constrained) = constrain_to_bounds(
            self.bounds.x,
            self.bounds.y,
            width.saturating_sub(self.bounds.width),
            height.saturating_sub(self.bounds.height),
        );
        self.bounds.x = x_constrained;
        self.bounds.y = y_constrained;
    }

    pub fn draw(&self, x: i32, y: i32) -> bool {
        self.bounds.contains_point(x, y)
    }

    pub fn input(&mut self, key_code: KeyCode, pressed: bool) {
        self.input_state.handle_key_state(key_code, pressed);
    }

    fn handle_input(&mut self) {
        if self.input_state.left_pressed {
            self.bounds.x -= self.speed;
        }
        if self.input_state.right_pressed {
            self.bounds.x += self.speed;
        }
        // if self.input_state.up_pressed {
        //     self.bounds.y -= self.speed;
        // }
        // if self.input_state.down_pressed {
        //     self.bounds.y += self.speed;
        // }
    }
}
