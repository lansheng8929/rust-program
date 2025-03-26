use rand::Rng;

use crate::{
    bullet::{Bullet, BulletOwner},
    enemy::Enemy,
    game_data::{self, GameData},
    player::Player,
    sound::{SoundEffect, SoundManager},
};

pub struct World {
    width: u32,
    height: u32,
    pub player: Option<Player>,
    pub enemies: Vec<Enemy>,
    pub bullets: Vec<Bullet>,
    pub sound_manager: SoundManager,
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            player: None,
            enemies: Vec::new(),
            bullets: Vec::new(),
            sound_manager: SoundManager::new(),
        }
    }

    pub fn update(&mut self, game_data: &mut GameData) {
        let player = self.player.as_mut().unwrap();
        player.update(self.width, self.height);

        // 更新所有敌人
        for enemy in self.enemies.iter_mut() {
            enemy.update(self.width, self.height);
        }

        // 处理玩家射击
        if let Some(new_bullet) = player.try_shoot() {
            self.bullets.push(new_bullet);
        }

        // 更新所有子弹
        let mut bullets_to_remove = Vec::new();
        for (i, bullet) in self.bullets.iter_mut().enumerate() {
            bullet.update();
            if bullet.is_out_of_bounds(self.width, self.height) {
                bullets_to_remove.push(i);
            }
        }

        // 移除出界的子弹
        for &i in bullets_to_remove.iter().rev() {
            self.bullets.remove(i);
        }
    }

    pub fn draw(&self, pixel: &mut [u8], x: i32, y: i32) {
        let player = self.player.as_ref().unwrap();

        // 绘制子弹
        if let Some(bullet) = self
            .bullets
            .iter()
            .find(|bullet| bullet.bounds.contains_point(x, y))
        {
            let color = match bullet.owner {
                BulletOwner::Player => [255, 255, 0, 255], // 玩家子弹黄色
                BulletOwner::Enemy => [255, 0, 0, 255],    // 敌人子弹红色
            };
            pixel.copy_from_slice(&color);
            return;
        }

        // 绘制敌人
        if let Some(enemy) = self
            .enemies
            .iter()
            .find(|enemy| enemy.bounds.contains_point(x, y))
        {
            let enemy_pixel = enemy.bounds.draw_pixel(x, y);
            pixel.copy_from_slice(&enemy_pixel);
            return;
        }

        // 绘制玩家
        if player.bounds.contains_point(x, y) {
            let player_pixel = player.bounds.draw_pixel(x, y);
            pixel.copy_from_slice(&player_pixel);
        }
        // 绘制背景
        else {
            pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
        }
    }

    pub fn set_player(&mut self, player: Player) {
        self.player = Some(player);
    }

    pub fn add_enemy(&mut self, enemy: Enemy) {
        self.enemies.push(enemy);
    }

    // 可以添加一个批量生成敌人的方法
    pub fn spawn_enemies(&mut self, count: u32) {
        let mut rng = rand::thread_rng();
        for _ in 0..count {
            let x = rng.gen_range(0..self.width) as i32;
            let y = rng.gen_range(0..self.height / 2) as i32; // 在上半部分生成敌人
            let enemy = Enemy::new(20, x, y, 2.0);
            self.enemies.push(enemy);
        }
    }
}
