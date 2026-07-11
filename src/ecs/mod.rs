use crate::ecs::{
    components::Component,
    components_manager::ComponentManager,
    entity_manager::{Entity, EntityManager},
};

pub mod components;
mod components_array;
mod components_manager;
mod entity_manager;

pub struct ECS {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
        }
    }
    pub fn create_entity(&mut self) {
        self.entity_manager.create_entity();
    }
    pub fn destroy_entity(&mut self, id: Entity) {
        self.entity_manager.destroy_entity(id);
    }
    pub fn alive_entities(&self) -> u32 {
        self.entity_manager.alive_entities()
    }
    pub fn is_alive(&self, id: Entity) -> bool {
        self.entity_manager.is_alive(id)
    }

    pub fn register_component<T: Component + 'static>(&mut self) {
        self.component_manager.register_component::<T>();
    }
    pub fn insert_component<T: Component + 'static>(&mut self, id: Entity) {
        let Some(component_type_ref) = self.component_manager.get_component_type::<T>() else {
            eprintln!("Failed to get component type");
            return;
        };
        let component_type = *component_type_ref;
        self.component_manager.insert_component::<T>(id);
        self.entity_manager.update_signature(id, component_type);
    }
    pub fn has_component<T: Component + 'static>(&self, id: Entity) -> bool {
        self.component_manager.has_component::<T>(id)
    }
}
