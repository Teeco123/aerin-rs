use crate::ecs::{components::Component, entity_manager::Entity};

pub trait IComponentArray {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

pub struct ComponentArray<T> {
    components: Vec<T>,
    dense: Vec<Entity>,
    sparse: Vec<usize>,
    size: usize,
}

impl<T: Component + 'static> IComponentArray for ComponentArray<T> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl<T: Component> ComponentArray<T> {
    pub fn new(max_entities: usize) -> Self {
        Self {
            components: Vec::with_capacity(max_entities),
            dense: vec![0; max_entities],
            sparse: vec![0; max_entities],
            size: 0,
        }
    }
    pub fn insert_component(&mut self, id: Entity) {
        let new_id = self.size;
        self.sparse[id as usize] = new_id;
        self.dense.push(id);
        self.components.push(T::default());
        self.size += 1;
    }

    pub fn has_component(&self, id: Entity) -> bool {
        let sparse = self.sparse.get(id as usize).unwrap();
        let dense = self.dense.get(*sparse).unwrap();
        if *dense == id {
            return true;
        }
        return false;
    }
}
