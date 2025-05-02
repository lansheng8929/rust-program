use ecs_rust::{
    component::Component,
    entity_manager::{EntityIdAccessor, EntityManager},
    system::System,
};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH, transform::Transform};

#[derive(Debug)]
pub struct CollisionBox {
    pub width: u32,
    pub height: u32,
    pub is_trigger: bool, // 可选：用于确定是否只是触发事件而不进行物理碰撞
}

impl Component for CollisionBox {}

impl CollisionBox {
    pub fn handle_wall_bounce(&self, transform: &Transform) -> (i32, i32) {
        let mut bounce_x = 0;
        let mut bounce_y = 0;

        if transform.position.0 <= 0 {
            bounce_x = -1;
        } else if transform.position.0 + self.width as i32 >= WINDOW_WIDTH as i32 {
            bounce_x = 1;
        }

        if transform.position.1 <= 0 {
            bounce_y = -1;
        } else if transform.position.1 + self.height as i32 >= WINDOW_HEIGHT as i32 {
            bounce_y = 1;
        }

        (bounce_x, bounce_y)
    }

    pub fn check_collision(
        &self,
        other: &CollisionBox,
        self_transform: &Transform,
        other_transform: &Transform,
    ) -> bool {
        let self_left = self_transform.position.0;
        let self_right = self_transform.position.0 + self.width as i32;
        let self_top = self_transform.position.1;
        let self_bottom = self_transform.position.1 + self.height as i32;

        let other_left = other_transform.position.0;
        let other_right = other_transform.position.0 + other.width as i32;
        let other_top = other_transform.position.1;
        let other_bottom = other_transform.position.1 + other.height as i32;

        !(self_right <= other_left
            || self_left >= other_right
            || self_bottom <= other_top
            || self_top >= other_bottom)
    }
}

pub struct CollisionSystem;

impl System for CollisionSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {}
}
