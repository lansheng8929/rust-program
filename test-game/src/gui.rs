use my_ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use my_ecs_rust::system::System;

pub struct GuiSystem {}

impl GuiSystem {}

impl System for GuiSystem {
    fn update(&mut self, manager: &mut EntityManager, _: &mut EntityIdAccessor) {}
}
