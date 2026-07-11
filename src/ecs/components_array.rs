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
