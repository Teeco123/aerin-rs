use std::cell::{Ref, RefMut};

use crate::component::Component;
use crate::entity::{Entity, EntityManager};
use crate::resource::Resources;
use crate::sparse_set::ComponentManager;
use crate::system::{SystemManager, SystemTrait};

pub struct ECS {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    system_manager: Option<SystemManager>,
    resources: Resources,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            system_manager: Some(SystemManager::new()),
            resources: Resources::new(),
        }
    }

    /// ===========
    ///  Resources
    /// ===========
    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(resource);
    }

    pub fn get_resource<T: 'static>(&self) -> Ref<'_, T> {
        self.resources.get::<T>().unwrap()
    }
    pub fn get_resource_mut<T: 'static>(&self) -> RefMut<'_, T> {
        self.resources.get_mut::<T>().unwrap()
    }

    /// ==========
    ///  Entities
    /// ==========
    pub fn create_entity(&mut self) -> Entity {
        self.entity_manager.create_entity()
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

    /// ============
    ///  Components
    /// ============
    pub fn register_component<T: Component + 'static>(&mut self) {
        self.component_manager.register_component::<T>();
    }

    pub fn insert_component<T: Component + 'static>(&mut self, id: Entity) {
        let Some(component_type_ref) = self.component_manager.get_component_type::<T>() else {
            return;
        };
        let component_type = *component_type_ref;
        self.component_manager.insert_component::<T>(id);

        let signature = self
            .entity_manager
            .update_signature(id, component_type)
            .unwrap();

        self.system_manager
            .as_mut()
            .unwrap()
            .update_system_entities(id, signature);
    }

    pub fn get_component<T: Component + 'static>(&self, id: Entity) -> Ref<'_, T> {
        self.component_manager.get_component(id).unwrap()
    }

    pub fn get_component_mut<T: Component + 'static>(&self, id: Entity) -> RefMut<'_, T> {
        self.component_manager.get_component_mut(id).unwrap()
    }

    pub fn has_component<T: Component + 'static>(&self, id: Entity) -> bool {
        self.component_manager.has_component::<T>(id)
    }

    /// =========
    ///  Systems
    /// =========
    pub fn register_system<T: SystemTrait + Default + 'static>(&mut self) {
        self.system_manager.as_mut().unwrap().register_system::<T>();
    }

    pub fn update_signature<T: SystemTrait + 'static, C: Component>(&mut self) {
        let component_type = self.component_manager.get_component_type::<C>().unwrap();
        self.system_manager
            .as_mut()
            .unwrap()
            .update_signature::<T>(*component_type);
    }

    /// =====
    ///  ECS
    /// =====
    pub fn start(&mut self) {
        let mut system_manager = self.system_manager.take().unwrap();
        system_manager.systems_start(self);
        self.system_manager = Some(system_manager);
    }

    pub fn update(&mut self) {
        let mut system_manager = self.system_manager.take().unwrap();
        system_manager.systems_update(self);
        self.system_manager = Some(system_manager);
    }
}
