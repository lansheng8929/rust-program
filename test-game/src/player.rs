use ecs_rust::{
    component::Component,
    entity_manager::{EntityIdAccessor, EntityManager},
    system::System,
};

use crate::{
    WINDOW_HEIGHT, WINDOW_WIDTH,
    collision_box::CollisionBox,
    transform::{self, Transform},
};

pub struct Player {
    pub name: &'static str,
}

impl Component for Player {}

pub struct PlayerSystem;

impl System for PlayerSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        if let Some(transform_ids) = accessor.borrow_ids_for_pair::<Transform, Player>(manager) {
            let mut updates = Vec::new();

            for transform_id in transform_ids {
                if let (Some(transform), Some(collision_box)) = (
                    manager.borrow_component::<Transform>(*transform_id),
                    manager.borrow_component::<CollisionBox>(*transform_id),
                ) {
                    let self_transform = transform;
                    let self_position = self_transform.position;
                    let mut self_velocity = self_transform.velocity;
                    let self_collision_box = collision_box;

                    let normal = self_collision_box.handle_wall_bounce(self_transform);
                    println!(
                        "Velocity: ({}, {}) Position: ({}, {})",
                        self_velocity.0, self_velocity.1, self_position.0, self_position.1
                    );
                    println!("normal:({}, {})", normal.0, normal.1);
                    if normal.0 != 0 || normal.1 != 0 {
                        if normal.0 != 0 && normal.1 != 0 {
                            // Handle corner collision: reverse both velocity components
                            self_velocity.0 = -self_velocity.0;
                            self_velocity.1 = -self_velocity.1;
                        } else {
                            let dot_product =
                                self_velocity.0 * normal.0 + self_velocity.1 * normal.1;
                            self_velocity.0 -= 2 * dot_product * normal.0;
                            self_velocity.1 -= 2 * dot_product * normal.1;
                        }
                    }

                    println!(
                        "goto:（{}，{}）",
                        self_position.0 + self_velocity.0,
                        self_position.1 + self_velocity.1
                    );

                    updates.push((
                        *transform_id,
                        self_velocity,
                        (
                            (self_position.0 + self_velocity.0)
                                .min(WINDOW_WIDTH as i32 - self_collision_box.width as i32),
                            (self_position.1 + self_velocity.1)
                                .min(WINDOW_HEIGHT as i32 - self_collision_box.height as i32),
                        ),
                    ));
                }
            }

            for (transform_id, new_velocity, new_position) in updates {
                if let Some(transform) = manager.borrow_component_mut::<Transform>(transform_id) {
                    transform.velocity = new_velocity;
                    transform.position = new_position;
                }
            }
        }
    }
}
