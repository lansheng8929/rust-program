use crate::rectangle::Rectangle;

#[derive(Debug, PartialEq)]
pub enum BulletOwner {
    Player,
    Enemy,
}

#[derive(Debug)]
pub struct Bullet {
    pub bounds: Rectangle,
    pub speed: f32,
    pub direction: f32,     // 弧度
    pub owner: BulletOwner, // 子弹所有者标识
}

impl Bullet {
    pub fn new(x: f32, y: f32, speed: f32, direction: f32, owner: BulletOwner) -> Self {
        let mut bounds = Rectangle::new(x, y, 4, 10); // 长方形子弹
        bounds.set_angle(direction);

        Self {
            bounds,
            speed,
            direction,
            owner,
        }
    }

    pub fn update(&mut self) {
        // 根据方向更新子弹位置
        self.bounds.x += (self.direction.cos() * self.speed) as f32;
        self.bounds.y += (self.direction.sin() * self.speed) as f32;
    }
}
