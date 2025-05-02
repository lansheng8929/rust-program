use ecs_rust::{
    component::Component,
    entity_manager::{ EntityIdAccessor, EntityManager },
    system::System,
};

use crate::{ transform::Transform, WINDOW_HEIGHT, WINDOW_WIDTH };

#[derive(Debug)]
pub struct CollisionBox {
    pub width: u32,
    pub height: u32,
    pub is_trigger: bool, // 可选：用于确定是否只是触发事件而不进行物理碰撞
}

impl Component for CollisionBox {}

impl CollisionBox {
    pub fn handle_wall_bounce(&self, x: &mut i32, y: &mut i32, vx: &mut i32, vy: &mut i32) {
        if *x <= 0 || *x + (self.width as i32) > (WINDOW_WIDTH as i32) {
            *vx = -*vx; // 反转水平速度
            // *x = (*x).clamp(0, (WINDOW_WIDTH - self.width) as i32); // 确保位置在窗口范围内
        }

        if *y <= 0 || (*y as u32) + self.height > WINDOW_HEIGHT {
            *vy = -*vy; // 反转垂直速度
            // *y = (*y).clamp(0, (WINDOW_HEIGHT - self.height) as i32); // 确保位置在窗口范围内
        }
    }
}

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {}
}
