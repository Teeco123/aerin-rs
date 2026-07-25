use crate::{
    app::AppResources,
    ecs::{
        components::Component,
        components_manager::ComponentManager,
        entity_manager::{Entity, EntityManager},
        system::SystemTrait,
        system_manager::SystemManager,
    },
};

pub mod components;
mod components_array;
mod components_manager;
pub mod entity_manager;
pub mod system;
mod system_manager;

pub struct ECS {
    entity_manager: EntityManager,
    component_manager: ComponentManager,
    system_manager: Option<SystemManager>,
}

impl ECS {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            component_manager: ComponentManager::new(),
            system_manager: Some(SystemManager::new()),
        }
    }
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

    pub fn register_component<T: Component + 'static>(&mut self) {
        self.component_manager.register_component::<T>();
    }

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

    pub fn get_component<T: Component + 'static>(&mut self, id: Entity) -> &mut T {
        self.component_manager.get_component(id).unwrap()
    }

    pub fn has_component<T: Component + 'static>(&self, id: Entity) -> bool {
        self.component_manager.has_component::<T>(id)
    }

    pub(crate) fn start(&mut self, res: &mut AppResources) {
        let mut system_manager = self.system_manager.take().unwrap();
        system_manager.systems_start(self, res);
        self.system_manager = Some(system_manager);
    }

    pub(crate) fn update(&mut self, res: &mut AppResources) {
        let mut system_manager = self.system_manager.take().unwrap();
        system_manager.systems_update(self, res);
        self.system_manager = Some(system_manager);
    }
}
