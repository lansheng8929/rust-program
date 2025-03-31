use std::collections::HashSet;

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

        // 使用 HashSet 来存储要移除的索引，避免重复
        let mut bullets_to_remove = HashSet::new();
        let mut enemies_to_remove = HashSet::new();

        // 更新所有敌人
        for (enemy_idx, enemy) in self.enemies.iter_mut().enumerate() {
            enemy.update(self.width, self.height);

            // 检查出界
            if enemy.bounds.is_out_of_bounds(self.width, self.height) {
                enemies_to_remove.insert(enemy_idx);
                continue;
            }
        }

        // 处理玩家射击 - 传入敌人引用
        if let Some(new_bullet) = player.try_shoot(&self.enemies) {
            self.bullets.push(new_bullet);
        }

        // 更新子弹并检查碰撞
        for (bullet_idx, bullet) in self.bullets.iter_mut().enumerate() {
            bullet.update();

            // 检查出界
            if bullet.bounds.is_out_of_bounds(self.width, self.height) {
                bullets_to_remove.insert(bullet_idx);
                continue; // 出界的子弹不需要再检查碰撞
            }

            // 检查碰撞
            if bullet.owner == BulletOwner::Player {
                for (enemy_idx, enemy) in self.enemies.iter_mut().enumerate() {
                    if bullet.bounds.is_overlapping(&enemy.bounds) {
                        bullets_to_remove.insert(bullet_idx);

                        // 对敌人造成伤害
                        if enemy.take_damage(bullet.damage) {
                            // 如果敌人死亡
                            enemies_to_remove.insert(enemy_idx);
                            game_data.score += 1;
                            self.sound_manager.play_sound(&SoundEffect::Collect);
                        }

                        break; // 一颗子弹只能击中一个敌人
                    }
                }
            }
        }

        // 按照索引从大到小的顺序移除，避免索引失效
        let mut bullets_to_remove: Vec<_> = bullets_to_remove.into_iter().collect();
        let mut enemies_to_remove: Vec<_> = enemies_to_remove.into_iter().collect();
        bullets_to_remove.sort_by(|a, b| b.cmp(a));
        enemies_to_remove.sort_by(|a, b| b.cmp(a));

        // 移除元素
        for &idx in &bullets_to_remove {
            self.bullets.remove(idx);
        }
        for &idx in &enemies_to_remove {
            self.enemies.remove(idx);
        }
    }

    pub fn draw(&self, pixel: &mut [u8], x: f32, y: f32) {
        let mut final_pixel = [0x00, 0x00, 0x00, 0x00];

        // 2. 绘制敌人
        for enemy in &self.enemies {
            if enemy.bounds.contains_point(x, y) {
                let mut enemy_pixel = enemy.bounds.draw(x, y);
                if enemy_pixel[3] > 0 {
                    // 根据血量调整颜色
                    let health_ratio = enemy.health / enemy.max_health;
                    enemy_pixel[0] = (enemy_pixel[0] as f32 * health_ratio) as u8;
                    enemy_pixel[1] = (enemy_pixel[1] as f32 * health_ratio) as u8;
                    enemy_pixel[2] = (enemy_pixel[2] as f32 * health_ratio) as u8;

                    // 如果当前像素不是完全透明的，进行 alpha 混合
                    if final_pixel[3] == 0 {
                        // 如果背景是完全透明的，直接使用新像素
                        final_pixel = enemy_pixel;
                    } else {
                        // 否则进行 alpha 混合
                        let alpha = enemy_pixel[3] as f32 / 255.0;
                        final_pixel[0] = (enemy_pixel[0] as f32 * alpha
                            + final_pixel[0] as f32 * (1.0 - alpha))
                            as u8;
                        final_pixel[1] = (enemy_pixel[1] as f32 * alpha
                            + final_pixel[1] as f32 * (1.0 - alpha))
                            as u8;
                        final_pixel[2] = (enemy_pixel[2] as f32 * alpha
                            + final_pixel[2] as f32 * (1.0 - alpha))
                            as u8;
                        final_pixel[3] = final_pixel[3].max(enemy_pixel[3]);
                    }
                }
            }
        }

        // 3. 绘制玩家
        let player = self.player.as_ref().unwrap();
        if player.bounds.contains_point(x, y) {
            let player_pixel = player.bounds.draw(x, y);
            if player_pixel[3] > 0 {
                // 只有当像素不完全透明时才更新
                final_pixel = player_pixel;
            }
        }

        // 4. 绘制子弹 (最上层)
        if let Some(bullet) = self
            .bullets
            .iter()
            .find(|bullet| bullet.bounds.contains_point(x, y))
        {
            let bullet_color = match bullet.owner {
                BulletOwner::Player => [255, 255, 0, 255], // 玩家子弹黄色
                BulletOwner::Enemy => [255, 0, 0, 255],    // 敌人子弹红色
            };
            final_pixel = bullet_color;
        }

        if final_pixel[3] == 0 {
            final_pixel = [0x00, 0x00, 0x00, 0xff];
        }

        pixel.copy_from_slice(&final_pixel);
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
            let x = rng.gen_range(0..self.width) as f32;
            let y = 0 as f32;
            let enemy = Enemy::new(20, x, y, 0.5);
            self.enemies.push(enemy);
        }
    }
}
