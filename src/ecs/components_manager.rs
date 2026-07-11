use std::collections::HashMap;

use crate::ecs::{
    components::{Component, ComponentType},
    components_array::{ComponentArray, IComponentArray},
    entity_manager::Entity,
};

pub struct ComponentManager {
    component_types: HashMap<&'static str, ComponentType>,
    components_arrays: HashMap<&'static str, Box<dyn IComponentArray>>,
    next_component_type: ComponentType,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            component_types: HashMap::new(),
            components_arrays: HashMap::new(),
            next_component_type: 0,
        }
    }
    pub fn register_component<T: Component + 'static>(&mut self) {
        let type_name = T::type_of();

        self.component_types
            .entry(type_name)
            .and_modify(|existing_value| {
                println!("DEBUG: Key already existed! Value is {}", existing_value);
            })
            .or_insert(self.next_component_type);

        self.components_arrays
            .entry(type_name)
            .or_insert(Box::new(ComponentArray::<T>::new(32)));

        self.next_component_type += 1;
    }
    pub fn insert_component<T: Component + 'static>(&mut self, id: Entity) {
        self.get_component_array_mut::<T>().insert_component(id);
    }

    pub fn has_component<T: Component + 'static>(&self, id: Entity) -> bool {
        self.get_component_array::<T>().has_component(id)
    }

    pub fn get_component_type<T: Component>(&self) -> &ComponentType {
        let type_name = T::type_of();
        assert!(
            self.component_types.contains_key(type_name),
            "Component not registered before use"
        );
        self.component_types.get(type_name).unwrap()
    }
}

impl ComponentManager {
    fn get_component_array<T: Component + 'static>(&self) -> &ComponentArray<T> {
        let type_name = T::type_of();

        assert!(
            self.component_types.contains_key(type_name),
            "Component not registered before use"
        );

        let boxed_trait = self.components_arrays.get(type_name).unwrap();

        boxed_trait
            .as_any()
            .downcast_ref::<ComponentArray<T>>()
            .expect("Failed to downcast to ComponentArray")
    }
    fn get_component_array_mut<T: Component + 'static>(&mut self) -> &mut ComponentArray<T> {
        let type_name = T::type_of();

        assert!(
            self.component_types.contains_key(type_name),
            "Component not registered before use"
        );

        let boxed_trait = self.components_arrays.get_mut(type_name).unwrap();

        boxed_trait
            .as_any_mut()
            .downcast_mut::<ComponentArray<T>>()
            .expect("Failed to downcast to ComponentArray")
    }
}
