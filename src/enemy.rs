use crate::uitils::{constrain_to_bounds, is_within_bounds};

pub(crate) struct Enemy {
    width: i16,
    height: i16,
    x: i16,
    y: i16,
    speed: i16,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            width: 10,
            height: 10,
            x: 10,
            y: 10,
            speed: 2,
        }
    }
}

impl Enemy {
    pub(crate) fn new(size: i16, x: i16, y: i16, speed: i16) -> Self {
        Self {
            width: size,
            height: size,
            x,
            y,
            speed,
        }
    }

    pub(crate) fn update(&mut self, width: u32, height: u32) {
        self.enemy_move(width, height);
    }

    pub(crate) fn draw(&self, x: i16, y: i16) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }

    fn enemy_move(&mut self, width: u32, height: u32) {
        let new_x = self.x + self.speed;
        let new_y = self.y + self.speed;

        let (x_constrained, y_constrained) = constrain_to_bounds(
            new_x as i32,
            new_y as i32,
            width.saturating_sub(self.width as u32),
            height.saturating_sub(self.height as u32),
        );

        self.x = x_constrained as i16;
        self.y = y_constrained as i16;
    }
}
