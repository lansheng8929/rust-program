use std::collections::HashMap;

use my_ecs_rust::{
    component::Component,
    entity_manager::{EntityIdAccessor, EntityManager},
    system::System,
};

use crate::{
    EntityTrait, WINDOW_HEIGHT, WINDOW_WIDTH,
    animation::Animation,
    collision_box::{self, CollisionBox},
    input::{self, Input},
    transform::{self, Transform},
    utils::get_assets_image_buffer,
};

#[derive(PartialEq, Eq, Hash)]
pub enum PlayerState {
    Idle,
    Walking,
}

pub struct Player {
    pub name: String,
    pub state: PlayerState,
    pub animations: HashMap<PlayerState, Animation>,
}

impl Component for Player {}

impl Player {
    pub fn new(name: Option<String>, state: Option<PlayerState>) -> Self {
        Player {
            name: if let Some(name) = name {
                name
            } else {
                "default".to_string()
            },
            state: if let Some(state) = state {
                state
            } else {
                PlayerState::Walking
            },

            animations: HashMap::from([
                (
                    PlayerState::Idle,
                    Animation::new(get_assets_image_buffer("player_idle", 32, 32, 4), 1000.0),
                ),
                (
                    PlayerState::Walking,
                    Animation::new(get_assets_image_buffer("player/walk/walk", 32, 32, 8), 1000.0),
                ),
            ]),
        }
    }
}

impl EntityTrait<PlayerState> for Player {
    fn set_state(&mut self, state: PlayerState) {
        self.state = state;
    }

    fn get_animation(&mut self) -> Option<&mut Animation> {
        self.animations.get_mut(&self.state)
    }
}

pub struct PlayerSystem;

impl System for PlayerSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        if let Some(players_ids) = accessor.borrow_ids::<Player>(manager) {
            let mut updates = Vec::new();

            for player_id in players_ids {
                if let (Some(transform), Some(collision_box)) = (
                    manager.borrow_component::<Transform>(*player_id),
                    manager.borrow_component::<CollisionBox>(*player_id),
                ) {
                    if let Some(input) = manager.get_resource::<Input>() {
                        let speed = 5.0;
                        let (mut goto_x, mut goto_y) = (transform.position.0, transform.position.1);

                        if input.left_pressed {
                            goto_x -= speed;
                        }
                        if input.right_pressed {
                            goto_x += speed;
                        }
                        // if input.up_pressed {
                        //     goto_y -= speed;
                        // }
                        // if input.down_pressed {
                        //     goto_y += speed;
                        // }

                        if input.left_pressed
                            || input.right_pressed
                            || input.up_pressed
                            || input.down_pressed
                        {
                            updates.push((
                                *player_id,
                                transform.velocity,
                                (
                                    goto_x.clamp(0.0, (WINDOW_WIDTH - collision_box.width) as f32),
                                    goto_y
                                        .clamp(0.0, (WINDOW_HEIGHT - collision_box.height) as f32),
                                ),
                            ));
                        }
                    }
                }
            }

            for (entity_id, new_velocity, new_position) in updates {
                if let Some(transform) = manager.borrow_component_mut::<Transform>(entity_id) {
                    transform.velocity = new_velocity;
                    transform.position = new_position;
                }
            }
        }
    }
}
