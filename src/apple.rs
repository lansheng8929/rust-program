use crate::rectangle::Rectangle;

pub(crate) struct Apple {
    pub(crate) bounds: Rectangle,
    speed: i32,
}

impl Default for Apple {
    fn default() -> Self {
        Self {
            bounds: Rectangle::new(10, 10, 10, 10),
            speed: 2,
        }
    }
}

impl Apple {
    pub(crate) fn new(size: u32, x: i32, y: i32, speed: i32) -> Self {
        Self {
            bounds: Rectangle::new(x, y, size, size),
            speed,
        }
    }

    pub(crate) fn update(&mut self) {
        self.bounds.y += self.speed
    }

    pub(crate) fn draw(&self, x: i32, y: i32) -> bool {
        self.bounds.contains_point(x, y)
    }
}
