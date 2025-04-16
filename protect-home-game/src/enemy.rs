use crate::rectangle::Rectangle;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum EnemyState {
    Idle,
    Moving,
}

#[derive(Debug)]
pub struct Enemy {
    pub bounds: Rectangle<EnemyState>,
    pub speed: f32,
    pub health: f32,     // 添加血量
    pub max_health: f32, // 添加最大血量
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            bounds: Rectangle::new(10.0, 10.0, 10, 10),
            speed: 2.0,
            health: 100.0,
            max_health: 100.0,
        }
    }
}

impl Enemy {
    pub fn new(size: u32, x: f32, y: f32, speed: f32) -> Self {
        let mut bounds = Rectangle::new(x, y, size, size);
        bounds.load_animation_state(EnemyState::Moving, "enemy_moving", 4);

        // 设置初始状态
        bounds.animation.set_state(EnemyState::Moving);
        bounds.animation.set_speed(10);

        Self {
            bounds,
            speed,
            health: 100.0, // 默认血量
            max_health: 100.0,
        }
    }

    pub fn update(&mut self) {
        self.bounds.y = (self.bounds.y as f32 + self.speed) as f32;

        // 更新动画
        self.bounds.animation.update();
    }

    // 添加受伤方法
    pub fn take_damage(&mut self, damage: f32) -> bool {
        self.health -= damage;
        self.health <= 0.0 // 返回是否死亡
    }
}
