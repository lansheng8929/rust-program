use crate::rectangle::Rectangle;

pub struct Apple {
    pub bounds: Rectangle,
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
    pub fn new(size: u32, x: i32, y: i32, speed: i32) -> Self {
        let mut bounds = Rectangle::new(x, y, size, size);
        bounds.load_texture("apple.png");
        Self { bounds, speed }
    }

    pub fn update(&mut self) {
        self.bounds.y += self.speed
    }

    pub fn draw(&self, x: i32, y: i32) -> bool {
        self.bounds.contains_point(x, y)
    }
}
