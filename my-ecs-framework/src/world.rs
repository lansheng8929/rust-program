use std::collections::HashMap;

use crate::{Archetype, Entity, EntityGenerator};

pub struct World {
    entity_generator: EntityGenerator, // 负责生成和管理实体ID
    archetypes: Vec<Archetype>,        // 存储多个不同的原型
}

impl World {
    pub fn new() -> Self {
        World {
            entity_generator: EntityGenerator::new(),
            archetypes: Vec::new(),
        }
    }

    // 创建实体
    pub fn spawn(&mut self) -> Entity {
        self.entity_generator.spawn()
    }

    // 销毁实体
    pub fn despawn(&mut self, entity: Entity) {
        self.entity_generator.despawn(entity);
    }
}
