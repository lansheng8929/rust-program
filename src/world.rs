use crate::{apple::Apple, enemy::Enemy, player::Player};

pub(crate) struct World {
    width: u32,
    height: u32,
    pub(crate) player: Option<Player>,
    pub(crate) enemy: Option<Enemy>,
    pub(crate) apple: Vec<Apple>,
}

impl World {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            player: None,
            enemy: None,
            apple: Vec::new(),
        }
    }

    pub(crate) fn update(&mut self) {
        self.player
            .as_mut()
            .unwrap()
            .update(self.width, self.height);

        self.enemy.as_mut().unwrap().update(self.width, self.height);

        self.apple
            .retain(|apple| apple.y + apple.height as i16 <= self.height as i16);
        for apple in self.apple.iter_mut() {
            apple.update(self.width, self.height);
        }
    }

    pub(crate) fn draw(&self, pixel: &mut [u8], x: i16, y: i16) {
        let inside_the_player = self.player.as_ref().unwrap().draw(x, y);
        let inside_the_enemy = self.enemy.as_ref().unwrap().draw(x, y);
        let mut inside_apple = false;
        for apple in self.apple.iter() {
            if apple.draw(x, y) {
                inside_apple = true;
                break;
            }
        }

        let rgba = if inside_the_player {
            [0x5e, 0x48, 0xe8, 0xff]
        } else if inside_the_enemy {
            [0xff, 0x00, 0x00, 0xff]
        } else if inside_apple {
            [0xff, 0x00, 0x00, 0xff]
        } else {
            [0x48, 0xb2, 0xe8, 0xff]
        };

        pixel.copy_from_slice(&rgba);
    }

    pub(crate) fn set_player(&mut self, player: Player) {
        self.player = Some(player);
    }

    pub(crate) fn set_enemy(&mut self, enemy: Enemy) {
        self.enemy = Some(enemy);
    }

    pub(crate) fn add_apple(&mut self, apple: Apple) {
        self.apple.push(apple)
    }
}
