use std::collections::VecDeque;

use crate::ecs::components::ComponentType;

pub type Entity = u32;
pub type EntitySignature = u32;

pub struct EntityManager {
    entities: u32,
    signatures: Vec<EntitySignature>,
    available: VecDeque<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: 0,
            signatures: vec![0; 32],
            available: VecDeque::new(),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        if let Some(id) = self.available.pop_front() {
            return id;
        }

        self.entities += 1;
        self.entities
    }

    pub fn destroy_entity(&mut self, id: Entity) {
        self.available.push_back(id);
    }

    pub fn update_signature(&mut self, id: Entity, r#type: ComponentType) {
        if let Some(value) = self.signatures.get_mut(id as usize) {
            *value = r#type as u32;
        }
    }

    pub fn alive_entities(&self) -> u32 {
        self.entities
    }

    pub fn is_alive(&self, id: Entity) -> bool {
        if id >= self.entities {
            return false;
        }

        if self.available.contains(&id) {
            return false;
        }

        return true;
    }
}
