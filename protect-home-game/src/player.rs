use std::f32::consts::PI;

use winit::keyboard::KeyCode;

use crate::{
    bullet::{Bullet, BulletOwner},
    enemy::Enemy,
    input_state::InputState,
    rectangle::Rectangle,
    uitils::constrain_to_bounds,
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayerState {
    Idle,
    Moving,
    Jumping,
    Shooting,
}

pub struct Player {
    pub bounds: Rectangle<PlayerState>,
    speed: f32,
    input_state: InputState,
    shoot_cooldown: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            bounds: Rectangle::new(10.0, 10.0, 10, 10),
            speed: 2.0,
            input_state: InputState::default(),
            shoot_cooldown: 0,
        }
    }
}

impl Player {
    pub fn new(size: u32, x: f32, y: f32, speed: f32) -> Self {
        let mut bounds = Rectangle::new(x, y, size, size);

        bounds.load_animation_state(PlayerState::Moving, "player_moving", 1);

        // 设置初始状态
        bounds.animation.set_state(PlayerState::Moving);
        bounds.animation.set_speed(10);

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

    pub fn input(&mut self, key_code: KeyCode, pressed: bool) {
        self.input_state.handle_key_state(key_code, pressed);
    }

    fn handle_input(&mut self) {
        if self.input_state.left_pressed {
            self.bounds.x -= self.speed as f32;
        }
        if self.input_state.right_pressed {
            self.bounds.x += self.speed as f32;
        }
        // if self.input_state.up_pressed {
        //     self.bounds.y -= self.speed;
        // }
        // if self.input_state.down_pressed {
        //     self.bounds.y += self.speed;
        // }
    }

    pub fn try_shoot(&mut self, enemies: &[Enemy]) -> Option<Bullet> {
        if
        // self.input_state.shoot_pressed &&
        self.shoot_cooldown == 0 {
            self.shoot_cooldown = 10;

            // 计算子弹的起始位置（从玩家中心发射）
            let bullet_start_x = self.bounds.x + (self.bounds.width as f32 / 2.0) as f32;
            let bullet_start_y = self.bounds.y;

            // 查找最近的敌人
            let target_angle = if let Some(closest_enemy) = self.find_closest_enemy(enemies) {
                // println!("（{},{}）", closest_enemy.bounds.x, closest_enemy.bounds.y);
                // 计算目标角度
                let enemy_center_x =
                    closest_enemy.bounds.x as f32 + closest_enemy.bounds.width as f32 / 2.0;
                let enemy_center_y =
                    closest_enemy.bounds.y as f32 + closest_enemy.bounds.height as f32 / 2.0;

                let dx = enemy_center_x - bullet_start_x as f32;
                let dy = enemy_center_y - bullet_start_y as f32;

                dy.atan2(dx)
            } else {
                // 如果没有敌人，默认向上射击
                -PI / 2.0
            };

            Some(Bullet::new(
                bullet_start_x,
                bullet_start_y,
                10.0,
                target_angle,
                BulletOwner::Player,
                20.0, // 设置子弹伤害值
            ))
        } else {
            if self.shoot_cooldown > 0 {
                self.shoot_cooldown -= 1;
            }
            None
        }
    }

    // 添加新方法来查找最近的敌人
    fn find_closest_enemy<'a>(&self, enemies: &'a [Enemy]) -> Option<&'a Enemy> {
        enemies.iter().min_by(|a, b| {
            let player_center_x = self.bounds.x + (self.bounds.width / 2) as f32;
            let player_center_y = self.bounds.y + (self.bounds.height / 2) as f32;

            let a_center_x = a.bounds.x + (a.bounds.width / 2) as f32;
            let a_center_y = a.bounds.y + (a.bounds.height / 2) as f32;
            let a_dx = a_center_x - player_center_x;
            let a_dy = a_center_y - player_center_y;
            let a_dist = a_dx * a_dx + a_dy * a_dy;

            let b_center_x = b.bounds.x + (b.bounds.width / 2) as f32;
            let b_center_y = b.bounds.y + (b.bounds.height / 2) as f32;
            let b_dx = b_center_x - player_center_x;
            let b_dy = b_center_y - player_center_y;
            let b_dist = b_dx * b_dx + b_dy * b_dy;

            a_dist.partial_cmp(&b_dist).unwrap()
        })
    }
}
