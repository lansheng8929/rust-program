use std::collections::HashMap;

use ecs_rust::{
    component::Component,
    entity_manager::{EntityIdAccessor, EntityManager},
    system::System,
};

use crate::{EntityTrait, animation::Animation, utils::get_assets_image_buffer};

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
                    Animation::new(get_assets_image_buffer("enemy_idle", 100, 100, 4), 1000.0),
                ),
                (
                    EntityState::Moving,
                    Animation::new(get_assets_image_buffer("enemy_moving", 100, 100, 4), 1000.0),
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

pub struct EntitySystem;

impl System for EntitySystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {}
}
