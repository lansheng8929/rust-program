pub(crate) struct Apple {
    pub(crate) width: i16,
    pub(crate) height: i16,
    pub(crate) x: i16,
    pub(crate) y: i16,
    speed: i16,
}

impl Default for Apple {
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

impl Apple {
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
        self.y += self.speed
    }

    pub(crate) fn draw(&self, x: i16, y: i16) -> bool {
        x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
    }
}
