use ecs_rust::component::Component;
use ecs_rust::entity_manager::{EntityIdAccessor, EntityManager};
use ecs_rust::system::System;
use winit::keyboard::KeyCode;

use crate::entity::{self, Entity};
use crate::get_pressed_keys;
use crate::player::Player;
use crate::transform::Transform;

#[derive(Debug, Clone, Copy)]
pub struct Input {
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub shoot_pressed: bool,
}

impl Component for Input {}

pub struct InputSystem {}

impl InputSystem {}

impl System for InputSystem {
    fn update(&mut self, manager: &mut EntityManager, accessor: &mut EntityIdAccessor) {
        let pressed_keys = get_pressed_keys();

        let ids = accessor
            .borrow_ids_for_pair::<Transform, Player>(manager)
            .unwrap();

        for id in ids.iter() {
            let player = manager.borrow_component::<Player>(*id).unwrap();

            if let Some(input) = manager.borrow_component_mut::<Input>(*id) {
                input.up_pressed = pressed_keys.contains(&KeyCode::KeyW);
                input.left_pressed = pressed_keys.contains(&KeyCode::KeyA);
                input.down_pressed = pressed_keys.contains(&KeyCode::KeyS);
                input.right_pressed = pressed_keys.contains(&KeyCode::KeyD);
                input.shoot_pressed = pressed_keys.contains(&KeyCode::Space);
            }
        }
    }
}
