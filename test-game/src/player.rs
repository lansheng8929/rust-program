use ecs_rust::{
    component::Component,
    entity_manager::{ EntityIdAccessor, EntityManager },
    system::System,
};

use crate::{
    collision_box::CollisionBox,
    transform::{ self, Transform },
    WINDOW_HEIGHT,
    WINDOW_WIDTH,
};

pub struct Player {
    pub name: &'static str,
}

impl Component for Player {}

pub struct PlayerSystem;

impl System for PlayerSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        if let Some(transform_ids) = accessor.borrow_ids_for_pair::<Transform, Player>(manager) {
            for transform_id in transform_ids {
                if let Some(transform) = manager.borrow_component_mut::<Transform>(*transform_id) {
                    let position = transform.position;
                    let mut velocity = transform.velocity;

                    // 可变借用结束，此时 `manager` 的可变借用已释放
                    if
                        let Some(collision_box) = manager.borrow_component::<CollisionBox>(
                            *transform_id
                        )
                    {
                        collision_box.handle_wall_bounce(
                            &mut (position.0 as i32),
                            &mut (position.1 as i32),
                            &mut velocity.0,
                            &mut velocity.1
                        );
                    }

                    // 更新 `Transform` 的数据
                    if
                        let Some(transform) = manager.borrow_component_mut::<Transform>(
                            *transform_id
                        )
                    {
                       
                        transform.velocity.0 = velocity.0;
                        transform.velocity.1 = velocity.1;
                        transform.position.0 += velocity.0;
                        transform.position.1 += velocity.1;
                    }
                }
            }
        }
    }
}
