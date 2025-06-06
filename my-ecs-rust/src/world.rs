use super::component::Component;
use super::entity_manager::{EntityIdAccessor, EntityManager};
use super::system::System;

pub struct World {
    entity_manager: EntityManager,
    entity_id_accessor: EntityIdAccessor,
    systems: Vec<Box<dyn System>>,
}

impl World {
    pub fn new() -> Self {
        World {
            entity_manager: EntityManager::new(),
            entity_id_accessor: EntityIdAccessor::new(),
            systems: vec![],
        }
    }

    pub fn create_entity(&mut self) -> usize {
        self.entity_manager.create_entity()
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        self.entity_manager.remove_entity(entity_id);
    }

    pub fn register_component<T: 'static + Component>(&mut self) -> &mut Self {
        self.entity_manager.register::<T>();
        self
    }

    pub fn add_system<T: 'static + System>(&mut self, system: T) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: T,
    ) -> &mut Self {
        self.entity_manager
            .add_component_to_entity(entity_id, component);
        self
    }

    pub fn add_resource<T: 'static>(&mut self, resource: T) -> &mut Self {
        self.entity_manager.add_resource(resource);
        self
    }

    pub fn get_resource<T: 'static>(&self) -> Option<&T> {
        self.entity_manager.get_resource()
    }

    pub fn get_resource_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.entity_manager.get_resource_mut()
    }

    pub fn remove_resource<T: 'static>(&mut self) -> Option<T> {
        self.entity_manager.remove_resource()
    }

    pub fn update(&mut self) {
        for system in self.systems.iter_mut() {
            system.update(&mut self.entity_manager, &mut self.entity_id_accessor);
            self.entity_manager.increment_frame();
        }
    }
}
