use crate::player::Player;

pub(crate) struct World {
    width: u32,
    height: u32,
    pub(crate) player: Option<Player>,
}

impl World {
    pub(crate) fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            player: None,
        }
    }

    pub(crate) fn update(&mut self) {
        self.player
            .as_mut()
            .unwrap()
            .update(self.width, self.height);
    }

    pub(crate) fn draw(&self, pixel: &mut [u8], x: i16, y: i16) {
        let inside_the_player = self.player.as_ref().unwrap().draw(x, y);

        let rgba = if inside_the_player {
            [0x5e, 0x48, 0xe8, 0xff]
        } else {
            [0x48, 0xb2, 0xe8, 0xff]
        };

        pixel.copy_from_slice(&rgba);
    }

    pub(crate) fn set_player(&mut self, player: Player) {
        self.player = Some(player);
    }
}
