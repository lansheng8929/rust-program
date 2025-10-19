

pub mod component;
pub mod component_manager;
pub mod entity;
pub mod entity_manager;
pub mod resource;
pub mod system;
pub mod world;
pub mod event;

pub mod prelude {

    pub use crate::{
        component::*,
        component_manager::*,
        entity::*,
        entity_manager::*,
        resource::*,
        system::*,
        world::*,
        event::*,
    };

}
