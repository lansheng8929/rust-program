use ecs_rust::{
    component::Component,
    entity_manager::{EntityIdAccessor, EntityManager},
    system::System,
};

use crate::{
    WINDOW_HEIGHT, WINDOW_WIDTH,
    entity::Entity,
    player::{self, Player},
    transform::Transform,
};

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
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let mut updates = Vec::new();

        let mut player_ids = Vec::<usize>::new();
        let mut entity_ids = Vec::<usize>::new();

        if let Some(_player_ids) = accessor.borrow_ids::<Player>(manager) {
            player_ids = _player_ids.clone();
        }

        if let Some(_entity_ids) = accessor.borrow_ids::<Entity>(manager) {
            entity_ids = _entity_ids.clone();
        }

        for entity_id in &entity_ids {
            if let (Some(transform), Some(collision_box)) = (
                manager.borrow_component::<Transform>(*entity_id),
                manager.borrow_component::<CollisionBox>(*entity_id),
            ) {
                let mut self_transform = transform.clone();

                // 处理与墙壁的碰撞
                let normal = collision_box.handle_wall_bounce(transform);
                if normal.0 != 0 || normal.1 != 0 {
                    if normal.0 != 0 && normal.1 != 0 {
                        self_transform.velocity.0 = -self_transform.velocity.0;
                        self_transform.velocity.1 = -self_transform.velocity.1;
                    } else {
                        let dot_product = self_transform.velocity.0 * normal.0
                            + self_transform.velocity.1 * normal.1;
                        self_transform.velocity.0 -= 2 * dot_product * normal.0;
                        self_transform.velocity.1 -= 2 * dot_product * normal.1;
                    }
                }

                // 处理与其他实体的碰撞
                for _entity_id in &entity_ids {
                    if _entity_id != entity_id {
                        if let (Some(other_transform), Some(other_collision_box)) = (
                            manager.borrow_component::<Transform>(*_entity_id),
                            manager.borrow_component::<CollisionBox>(*_entity_id),
                        ) {
                            if collision_box.check_collision(
                                other_collision_box,
                                transform,
                                other_transform,
                            ) {
                                self_transform.velocity.0 = -self_transform.velocity.0;
                                self_transform.velocity.1 = -self_transform.velocity.1;
                            }
                        }
                    }
                }

                // 处理与玩家的碰撞
                for _player_id in &player_ids {
                    if let (Some(player_transform), Some(player_collision_box)) = (
                        manager.borrow_component::<Transform>(*_player_id),
                        manager.borrow_component::<CollisionBox>(*_player_id),
                    ) {
                        if collision_box.check_collision(
                            player_collision_box,
                            transform,
                            player_transform,
                        ) {
                            self_transform.velocity.0 = -self_transform.velocity.0;
                            self_transform.velocity.1 = -self_transform.velocity.1;
                        }
                    }
                }

                updates.push((
                    *entity_id,
                    self_transform.velocity,
                    (
                        (self_transform.position.0 + self_transform.velocity.0)
                            .min(WINDOW_WIDTH as i32 - collision_box.width as i32),
                        (self_transform.position.1 + self_transform.velocity.1)
                            .min(WINDOW_HEIGHT as i32 - collision_box.height as i32),
                    ),
                ));
            }

            // 更新位置
            for (entity_id, new_velocity, new_position) in &updates {
                if let Some(transform) = manager.borrow_component_mut::<Transform>(*entity_id) {
                    transform.velocity = *new_velocity;
                    transform.position = *new_position;
                }
            }
        }
    }
}
