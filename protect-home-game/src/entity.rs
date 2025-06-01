use rand::Rng;
use std::collections::HashMap;

use my_ecs_rust::{
    component::Component,
    entity_manager::{EntityIdAccessor, EntityManager},
    system::System,
};

use crate::{
    EntityTrait, WINDOW_HEIGHT, WINDOW_WIDTH, animation::Animation, collision_box::CollisionBox,
    get_delta_time, transform::Transform, utils::get_assets_image_buffer,
};

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum EntityState {
    Idle,
    Moving,
}

#[derive(Debug)]
pub struct Entity {
    pub name: String,
    pub state: EntityState,
    pub animations: HashMap<EntityState, Animation>,
}

impl Component for Entity {}

impl Entity {
    pub fn new(name: Option<String>, state: Option<EntityState>) -> Self {
        Entity {
            name: if let Some(name) = name {
                name
            } else {
                "default".to_string()
            },
            state: if let Some(state) = state {
                state
            } else {
                EntityState::Idle
            },

            animations: HashMap::from([
                (
                    EntityState::Idle,
                    Animation::new(get_assets_image_buffer("enemy_idle", 32, 32, 4), 1000.0),
                ),
                (
                    EntityState::Moving,
                    Animation::new(get_assets_image_buffer("enemy_moving", 32, 32, 4), 1000.0),
                ),
            ]),
        }
    }
}

impl EntityTrait<EntityState> for Entity {
    fn set_state(&mut self, state: EntityState) {
        self.state = state;
    }

    fn get_animation(&mut self) -> Option<&mut Animation> {
        self.animations.get_mut(&self.state)
    }
}

pub struct EntitySystem {
    pub enemy_spawn_timer: f32,
}

impl EntitySystem {
    pub fn new() -> Self {
        EntitySystem {
            enemy_spawn_timer: 0.0,
        }
    }

    fn spawn_random_enemies(&self, manager: &mut EntityManager) {
        let mut rng = rand::thread_rng();
        let enemy_count = rng.gen_range(1..10);

        for i in 0..enemy_count {
            let entity_id = manager.create_entity();

            // Generate random position at the top of the screen
            let random_x = rng.gen_range(0..WINDOW_WIDTH - 50);
            let random_y = -10;

            manager.add_component_to_entity(
                entity_id,
                Entity::new(
                    Some(format!("enemy-{}", i).to_string()),
                    Some(EntityState::Moving),
                ),
            );
            manager.add_component_to_entity(
                entity_id,
                Transform {
                    position: (random_x as f32, random_y as f32),
                    velocity: (0.0, rng.gen_range(1.0..2.0)),
                    direction: (0.0, 1.0),
                },
            );
            manager.add_component_to_entity(
                entity_id,
                CollisionBox {
                    width: 32,
                    height: 32,
                },
            );
        }
    }

    fn remove_out_of_bounds_entities(
        &self,
        manager: &mut EntityManager,
        accessor: &mut EntityIdAccessor,
    ) {
        if let Some(entity_ids) = accessor.borrow_ids::<Entity>(manager) {
            let mut entities_to_remove = Vec::new();

            for entity_id in entity_ids {
                if let Some(transform) = manager.borrow_component::<Transform>(*entity_id) {
                    // Check if entity is below the bottom of the screen
                    if transform.position.1 > WINDOW_HEIGHT as f32 {
                        entities_to_remove.push(*entity_id);
                    }
                }
            }

            // Remove entities that are off-screen
            for entity_id in entities_to_remove {
                manager.remove_entity(entity_id);
            }
        }
    }
}

impl System for EntitySystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let delta_time = get_delta_time();

        // Increment spawn timer
        self.enemy_spawn_timer += delta_time;

        // Spawn enemies every second
        if self.enemy_spawn_timer >= 1000.0 {
            self.enemy_spawn_timer = 0.0;
            self.spawn_random_enemies(manager);
        }

        // Destroy enemies
        self.remove_out_of_bounds_entities(manager, accessor);

        if let Some(entity_ids) = accessor.borrow_ids::<Entity>(manager) {
            let mut updates = Vec::new();

            for entity_id in entity_ids {
                if let (Some(transform), Some(collision_box)) = (
                    manager.borrow_component::<Transform>(*entity_id),
                    manager.borrow_component::<CollisionBox>(*entity_id),
                ) {
                    let self_transform = transform.clone();

                    updates.push((
                        *entity_id,
                        self_transform.velocity,
                        (
                            (self_transform.position.0)
                                .clamp(0.0, (WINDOW_WIDTH - collision_box.width) as f32),
                            (self_transform.position.1 + self_transform.velocity.1), // .clamp(0, WINDOW_HEIGHT as i32 - collision_box.height as i32),
                        ),
                    ));
                }
            }

            for (entity_id, new_velocity, new_position) in &updates {
                if let Some(transform) = manager.borrow_component_mut::<Transform>(*entity_id) {
                    // transform.velocity = *new_velocity;
                    transform.position = *new_position;
                }
            }
        }
    }
}
