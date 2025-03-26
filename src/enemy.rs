use crate::rectangle::Rectangle;

pub struct Enemy {
    pub bounds: Rectangle,
    speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            bounds: Rectangle::new(10, 10, 10, 10),
            speed: 2.0,
        }
    }
}

impl Enemy {
    pub fn new(size: u32, x: i32, y: i32, speed: f32) -> Self {
        let mut bounds = Rectangle::new(x, y, size, size);
        bounds.load_texture("assets/enemy.png");

        Self { bounds, speed }
    }

    pub fn update(&mut self, width: u32, height: u32) {
        self.bounds.y = (self.bounds.y as f32 + self.speed) as i32;
        if self.bounds.y > height as i32 {
            self.bounds.y = 0;
            self.bounds.x = rand::random::<i32>() % width as i32;
        }
    }

    pub fn draw(&self, x: i32, y: i32) -> bool {
        self.bounds.contains_point(x, y)
    }
}
