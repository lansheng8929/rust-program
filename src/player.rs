use std::f32::consts::PI;

use winit::keyboard::KeyCode;

use crate::{
    bullet::{Bullet, BulletOwner},
    input_state::InputState,
    rectangle::Rectangle,
    uitils::constrain_to_bounds,
};

pub struct Player {
    pub bounds: Rectangle,
    speed: f32,
    input_state: InputState,
    shoot_cooldown: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            bounds: Rectangle::new(10, 10, 10, 10),
            speed: 2.0,
            input_state: InputState::default(),
            shoot_cooldown: 0,
        }
    }
}

impl Player {
    pub fn new(size: u32, x: i32, y: i32, speed: f32) -> Self {
        let mut bounds = Rectangle::new(x, y, size, size);
        bounds.load_texture("player.png");
        Self {
            bounds,
            speed,
            input_state: InputState::default(),
            shoot_cooldown: 0,
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
            self.bounds.x -= self.speed as i32;
        }
        if self.input_state.right_pressed {
            self.bounds.x += self.speed as i32;
        }
        // if self.input_state.up_pressed {
        //     self.bounds.y -= self.speed;
        // }
        // if self.input_state.down_pressed {
        //     self.bounds.y += self.speed;
        // }
    }

    pub fn try_shoot(&mut self) -> Option<Bullet> {
        if self.input_state.shoot_pressed && self.shoot_cooldown == 0 {
            self.shoot_cooldown = 10;
            Some(Bullet::new(
                self.bounds.x + (self.bounds.width as i32 / 2),
                self.bounds.y,
                5.0,
                -PI / 2.0,
                BulletOwner::Player,
            ))
        } else {
            if self.shoot_cooldown > 0 {
                self.shoot_cooldown -= 1;
            }
            None
        }
    }
}
