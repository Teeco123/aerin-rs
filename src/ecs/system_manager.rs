use std::collections::HashMap;

use crate::{
    app::AppResources,
    ecs::{
        ECS,
        components::ComponentType,
        entity_manager::{Entity, EntitySignature},
        system::{System, SystemSignature, SystemTrait},
    },
};

pub struct SystemManager {
    systems: HashMap<&'static str, System>,
    signatures: HashMap<&'static str, SystemSignature>,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
            signatures: HashMap::new(),
        }
    }
    pub fn register_system<T: SystemTrait + Default + 'static>(&mut self) {
        let type_name = T::type_of();

        if self.systems.contains_key(type_name) {
            return;
        }

        let system = System {
            entities: Vec::new(),
            system: Box::new(T::new()),
        };

        self.systems.insert(type_name, system);
        self.signatures.insert(type_name, 0);
    }

    pub fn update_signature<T: SystemTrait + 'static>(&mut self, component_type: ComponentType) {
        if let Some(value) = self.signatures.get_mut(T::type_of()) {
            *value = *value ^ (1 << component_type);
        }
    }

    pub fn update_system_entities(&mut self, entity: Entity, entity_signature: EntitySignature) {
        for (name, system) in &mut self.systems {
            let sig = self.signatures.get(name).unwrap();
            if (entity_signature & *sig) == *sig {
                if !system.entities.contains(&entity) {
                    system.entities.push(entity);
                }
            } else {
                if let Some(index) = system.entities.iter().position(|&e| e == entity) {
                    system.entities.swap_remove(index);
                }
            }
        }
    }

    pub fn systems_update(&mut self, ecs: &mut ECS, res: &mut AppResources) {
        for (_name, system) in &mut self.systems {
            system.system.update(&mut system.entities, ecs, res);
        }
    }

    #[allow(dead_code)]
    pub fn systems_fixed_update(&mut self) {
        for (_name, system) in &mut self.systems {
            system.system.fixed_update(&mut system.entities);
        }
    }
}
