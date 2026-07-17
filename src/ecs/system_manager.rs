use std::collections::HashMap;

use crate::ecs::system::{System, SystemSignature, SystemTrait};

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
    }

    pub fn systems_update(&mut self) {
        for (_name, system) in &mut self.systems {
            system.system.update(&mut system.entities);
        }
    }

    pub fn systems_fixed_update(&mut self) {
        for (_name, system) in &mut self.systems {
            system.system.fixed_update(&mut system.entities);
        }
    }
}
