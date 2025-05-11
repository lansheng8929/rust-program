use crate::rectangle::Rectangle;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum BulletState {
    Shooting,
}

#[derive(Debug, PartialEq)]
pub enum BulletOwner {
    Entity,
    Enemy,
}

#[derive(Debug)]
pub struct Bullet {
    pub bounds: Rectangle<BulletState>,
    pub speed: f32,
    pub direction: f32,     // 弧度
    pub owner: BulletOwner, // 子弹所有者标识
    pub damage: f32,        // 添加伤害值
}

impl Bullet {
    pub fn new(
        x: f32,
        y: f32,
        speed: f32,
        direction: f32,
        owner: BulletOwner,
        damage: f32,
    ) -> Self {
        let mut bounds = Rectangle::new(x, y, 4, 10);
        bounds.set_angle(direction);

        Self {
            bounds,
            speed,
            direction,
            owner,
            damage,
        }
    }

    pub fn update(&mut self) {
        // 根据方向更新子弹位置
        self.bounds.x += (self.direction.cos() * self.speed) as f32;
        self.bounds.y += (self.direction.sin() * self.speed) as f32;
    }
}
