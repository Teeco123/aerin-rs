use std::any::Any;
use std::collections::HashMap;

use crate::component::Component;
use crate::entity::{ComponentType, Entity};

pub trait IComponentArray {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ComponentArray<T> {
    components: Vec<T>,
    dense: Vec<Entity>,
    sparse: Vec<usize>,
    size: usize,
}

impl<T: Component + 'static> IComponentArray for ComponentArray<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl<T: Component> ComponentArray<T> {
    pub fn new(max_entities: usize) -> Self {
        Self {
            components: Vec::with_capacity(max_entities),
            dense: Vec::with_capacity(max_entities),
            sparse: vec![usize::MAX; max_entities],
            size: 0,
        }
    }

    pub fn insert_component(&mut self, id: Entity) {
        let id_usize = id as usize;
        if id_usize >= self.sparse.len() {
            self.sparse.resize(self.sparse.len() + 5, usize::MAX);
        }

        self.sparse[id_usize] = self.size;
        self.dense.push(id);
        self.components.push(T::default());
        self.size += 1;
    }

    pub fn get_component(&mut self, id: Entity) -> &mut T {
        let dense_index = self.sparse.get(id as usize).unwrap();
        self.components.get_mut(*dense_index).unwrap()
    }

    pub fn has_component(&self, id: Entity) -> bool {
        if id as usize >= self.sparse.len() {
            return false;
        }
        let dense_index = self.sparse[id as usize];
        if dense_index >= self.size {
            return false;
        }
        self.dense[dense_index] == id
    }
}

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
        if self.component_types.contains_key(type_name) {
            return;
        }
        self.component_types
            .insert(type_name, self.next_component_type);
        self.components_arrays
            .insert(type_name, Box::new(ComponentArray::<T>::new(32)));
        self.next_component_type += 1;
    }

    pub fn insert_component<T: Component + 'static>(&mut self, id: Entity) {
        if let Some(array) = self.get_component_array_mut::<T>() {
            array.insert_component(id);
        }
    }

    pub fn get_component<T: Component + 'static>(&mut self, id: Entity) -> Option<&mut T> {
        let array = self.get_component_array_mut::<T>()?;
        Some(array.get_component(id))
    }

    pub fn has_component<T: Component + 'static>(&self, id: Entity) -> bool {
        if let Some(array) = self.get_component_array::<T>() {
            array.has_component(id)
        } else {
            false
        }
    }

    pub fn get_component_type<T: Component>(&self) -> Option<&ComponentType> {
        self.component_types.get(T::type_of())
    }

    fn get_component_array<T: Component + 'static>(&self) -> Option<&ComponentArray<T>> {
        let boxed = self.components_arrays.get(T::type_of())?;
        boxed.as_any().downcast_ref::<ComponentArray<T>>()
    }

    fn get_component_array_mut<T: Component + 'static>(
        &mut self,
    ) -> Option<&mut ComponentArray<T>> {
        let boxed = self.components_arrays.get_mut(T::type_of())?;
        boxed.as_any_mut().downcast_mut::<ComponentArray<T>>()
    }
}
