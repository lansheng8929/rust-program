use std::collections::HashMap;

use my_ecs_rust::{
    component::Component,
    entity_manager::{EntityIdAccessor, EntityManager},
    system::System,
};

use crate::{
    EntityTrait, GameState, WINDOW_HEIGHT, WINDOW_WIDTH,
    animation::Animation,
    collision_box::CollisionBox,
    entity::Entity,
    input::Input,
    player::Player,
    sound::{SoundEffect, SoundSystem},
    transform::{self, Transform},
    utils::get_assets_image_buffer,
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BulletState {
    Moving,
}

#[derive(Debug, Clone)]
pub struct Bullet {
    pub state: BulletState,
    pub animations: HashMap<BulletState, Animation>,
}

impl Component for Bullet {}

impl Bullet {
    pub fn new(state: Option<BulletState>) -> Self {
        Bullet {
            state: if let Some(state) = state {
                state
            } else {
                BulletState::Moving
            },

            animations: HashMap::from([(
                BulletState::Moving,
                Animation::new(get_assets_image_buffer("bullet_moving", 32, 32, 4), 1000.0),
            )]),
        }
    }
}

impl EntityTrait<BulletState> for Bullet {
    fn set_state(&mut self, state: BulletState) {
        self.state = state;
    }

    fn get_animation(&mut self) -> Option<&mut Animation> {
        self.animations.get_mut(&self.state)
    }
}

pub struct BulletSystem {
    pub bullet_spawn_timer: f32,
}

impl BulletSystem {
    pub fn new() -> Self {
        BulletSystem {
            bullet_spawn_timer: 0.0,
        }
    }

    fn spawn_bullet(
        &self,
        manager: &mut EntityManager,
        accessor: &mut EntityIdAccessor,
        player_id: &usize,
    ) {
        let bullet_id = manager.create_entity();

        if let Some(player_transform) = manager.borrow_component::<Transform>(*player_id) {
            let player_position = player_transform.position;

            let mut closest_entity_position = None;
            let mut closest_distance_squared = f32::MAX;

            if let Some(entity_ids) = accessor.borrow_ids::<Entity>(manager) {
                for entity_id in entity_ids.clone() {
                    if let Some(entity_transform) = manager.borrow_component::<Transform>(entity_id)
                    {
                        let dx = entity_transform.position.0 - player_position.0;
                        let dy = entity_transform.position.1 - player_position.1;
                        let distance_squared = (dx * dx + dy * dy) as f32;

                        if distance_squared < closest_distance_squared {
                            closest_distance_squared = distance_squared;
                            closest_entity_position = Some(entity_transform.position);
                        }
                    }
                }
            }

            let speed = 10.0;
            let velocity = if let Some(target_position) = closest_entity_position {
                let dx = target_position.0 - player_position.0;
                let dy = target_position.1 - player_position.1;
                let magnitude = ((dx * dx + dy * dy) as f32).sqrt();
                if magnitude > 0.0 {
                    (dx / magnitude * speed, dy / magnitude * speed)
                } else {
                    (0.0, -1.0 * speed)
                }
            } else {
                (0.0, -1.0 * speed)
            };

            let magnitude = (velocity.0 * velocity.0 + velocity.1 * velocity.1).sqrt();
            let direction = if magnitude > 0.0 {
                (velocity.0 / magnitude, velocity.1 / magnitude)
            } else {
                (0.0, -1.0)
            };

            manager.add_component_to_entity(bullet_id, Bullet::new(Some(BulletState::Moving)));

            manager.add_component_to_entity(
                bullet_id,
                Transform {
                    position: player_position,
                    velocity,
                    direction,
                },
            );

            manager.add_component_to_entity(
                bullet_id,
                CollisionBox {
                    width: 32,
                    height: 32,
                },
            );
        }

        if let Some(sound_system) = manager.get_resource_mut::<SoundSystem>() {
            sound_system.play_sound(&SoundEffect::SHOOT, Some(0.5));
        }
    }
}

impl System for BulletSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let delta_time = if let Some(game_state) = manager.get_resource::<GameState>() {
            game_state.delta_time
        } else {
            16.67 // fallback to ~60fps if GameState is not available
        };

        self.bullet_spawn_timer += delta_time;

        let player_ids = accessor.borrow_ids::<Player>(manager);
        if let Some(player_ids) = player_ids {
            for player_id in player_ids.clone() {
                if let Some(input) = manager.get_resource::<Input>() {
                    if input.shoot_pressed {
                        if self.bullet_spawn_timer >= 100.0 {
                            self.bullet_spawn_timer = 0.0;
                            self.spawn_bullet(manager, accessor, &player_id);
                        }
                    }
                }
            }
        }

        let bullet_ids = accessor.borrow_ids::<Bullet>(manager).cloned();
        let entity_ids = accessor.borrow_ids::<Entity>(manager).cloned();
        if let (Some(bullet_ids), Some(entity_ids)) = (bullet_ids, entity_ids) {
            for bullet_id in bullet_ids {
                if let Some(transform) = manager.borrow_component::<Transform>(bullet_id) {
                    let mut new_position = transform.position;
                    new_position.0 += transform.velocity.0;
                    new_position.1 += transform.velocity.1;

                    if new_position.1 < 0.0 {
                        manager.remove_entity(bullet_id);
                    } else {
                        if let Some(transform) =
                            manager.borrow_component_mut::<Transform>(bullet_id)
                        {
                            transform.position = new_position;
                        }
                    }
                }

                for entity_id in &entity_ids {
                    if let (
                        Some(bullet_transform),
                        Some(bullet_collision_box),
                        Some(entity_transform),
                        Some(entity_collision_box),
                    ) = (
                        manager.borrow_component::<Transform>(bullet_id),
                        manager.borrow_component::<CollisionBox>(bullet_id),
                        manager.borrow_component::<Transform>(*entity_id),
                        manager.borrow_component::<CollisionBox>(*entity_id),
                    ) {
                        if bullet_collision_box.check_collision(
                            entity_collision_box,
                            bullet_transform,
                            entity_transform,
                        ) {
                            manager.remove_entity(bullet_id);
                            manager.remove_entity(*entity_id);

                            if let Some(sound_system) = manager.get_resource_mut::<SoundSystem>() {
                                sound_system.play_sound(&SoundEffect::SCORE, None);
                            }

                            if let Some(game_state) = manager.get_resource_mut::<GameState>() {
                                game_state.score += 1;
                            }
                        }
                    }
                }
            }
        }
    }
}
