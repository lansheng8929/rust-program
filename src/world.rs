use crate::{
    apple::Apple,
    enemy::Enemy,
    game_data::{self, GameData},
    player::Player,
};

pub struct World {
    width: u32,
    height: u32,
    pub player: Option<Player>,
    pub enemy: Option<Enemy>,
    pub apple: Vec<Apple>,
}

impl World {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            player: None,
            enemy: None,
            apple: Vec::new(),
        }
    }

    pub fn update(&mut self, game_data: &mut GameData) {
        let player = self.player.as_mut().unwrap();

        player.update(self.width, self.height);

        // self.enemy.as_mut().unwrap().update(self.width, self.height);

        // 玩家接住苹果删除苹果
        let mut to_remove: Vec<usize> = Vec::new();
        for (i, apple) in self.apple.iter().enumerate() {
            // 超出屏幕底部
            if apple.bounds.y as u32 + apple.bounds.height > self.height {
                to_remove.push(i);
                continue;
            }

            // 与玩家碰撞
            if apple.bounds.is_overlapping(&player.bounds) {
                game_data.score += 1;
                to_remove.push(i);
            }
        }
        // 从后向前移除元素
        for &i in to_remove.iter().rev() {
            self.apple.remove(i);
        }

        for apple in self.apple.iter_mut() {
            apple.update();
        }
    }

    pub fn draw(&self, pixel: &mut [u8], x: i32, y: i32) {
        let player = self.player.as_ref().unwrap();
        let apple_pixel = self
            .apple
            .iter()
            .find(|apple| apple.bounds.contains_point(x, y))
            .map(|apple| apple.bounds.draw_pixel(x, y));

        if player.bounds.contains_point(x, y) {
            let player_pixel = player.bounds.draw_pixel(x, y);
            pixel.copy_from_slice(&player_pixel);
        } else if let Some(apple_pixel) = apple_pixel {
            pixel.copy_from_slice(&apple_pixel);
        } else {
            pixel.copy_from_slice(&[0x00, 0x00, 0x00, 0xff]);
        }
    }

    pub fn set_player(&mut self, player: Player) {
        self.player = Some(player);
    }

    pub fn set_enemy(&mut self, enemy: Enemy) {
        self.enemy = Some(enemy);
    }

    pub fn add_apple(&mut self, apple: Apple) {
        self.apple.push(apple)
    }
}
