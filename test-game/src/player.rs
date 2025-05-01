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
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {}
}
