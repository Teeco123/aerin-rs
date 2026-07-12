use std::{any::type_name, collections::HashMap};

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
            .and_modify(|_| {
                println!("Component {} is already registered", type_name);
            })
            .or_insert(self.next_component_type);

        self.components_arrays
            .entry(type_name)
            .or_insert(Box::new(ComponentArray::<T>::new(32)));

        self.next_component_type += 1;
    }
    pub fn insert_component<T: Component + 'static>(&mut self, id: Entity) {
        if let Some(array) = self.get_component_array_mut::<T>() {
            array.insert_component(id);
        } else {
            eprintln!("Failed while inserting component");
        }
    }

    pub fn has_component<T: Component + 'static>(&self, id: Entity) -> bool {
        if let Some(array) = self.get_component_array::<T>() {
            array.has_component(id)
        } else {
            false
        }
    }

    pub fn get_component_type<T: Component>(&self) -> Option<&ComponentType> {
        let type_name = T::type_of();
        if !self.component_types.contains_key(type_name) {
            eprintln!("Component {} not registered before use", type_name);
            return None;
        }
        Some(self.component_types.get(type_name).unwrap())
    }
}

impl ComponentManager {
    fn get_component_array<T: Component + 'static>(&self) -> Option<&ComponentArray<T>> {
        let type_name = T::type_of();

        let boxed_trait = self.components_arrays.get(type_name)?;

        Some(
            boxed_trait
                .as_any()
                .downcast_ref::<ComponentArray<T>>()
                .expect("Failed to downcast to ComponentArray"),
        )
    }
    fn get_component_array_mut<T: Component + 'static>(
        &mut self,
    ) -> Option<&mut ComponentArray<T>> {
        let type_name = T::type_of();

        let boxed_trait = self.components_arrays.get_mut(type_name)?;

        Some(
            boxed_trait
                .as_any_mut()
                .downcast_mut::<ComponentArray<T>>()
                .expect("Failed to downcast to ComponentArray"),
        )
    }
}
