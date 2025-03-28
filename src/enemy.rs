use crate::rectangle::Rectangle;

#[derive(Debug)]
pub struct Enemy {
    pub bounds: Rectangle,
    speed: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            bounds: Rectangle::new(10.0, 10.0, 10, 10),
            speed: 2.0,
        }
    }
}

impl Enemy {
    pub fn new(size: u32, x: f32, y: f32, speed: f32) -> Self {
        let mut bounds = Rectangle::new(x, y, size, size);
        bounds.load_texture("enemy.png");

        Self { bounds, speed }
    }

    pub fn update(&mut self, width: u32, height: u32) {
        self.bounds.y = (self.bounds.y as f32 + self.speed) as f32;
    }

    pub fn draw(&self, x: f32, y: f32) -> bool {
        self.bounds.contains_point(x, y)
    }
}
