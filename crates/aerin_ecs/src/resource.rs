use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
};

#[derive(Default)]
pub struct Resources {
    storage: HashMap<TypeId, RefCell<Box<dyn Any>>>,
}

impl Resources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<T: 'static>(&mut self, resource: T) {
        self.storage
            .insert(TypeId::of::<T>(), RefCell::new(Box::new(resource)));
    }

    pub fn get<T: 'static>(&self) -> Option<Ref<'_, T>> {
        let cell = self.storage.get(&TypeId::of::<T>())?;

        Some(Ref::map(cell.borrow(), |boxed| {
            boxed.downcast_ref::<T>().unwrap()
        }))
    }

    pub fn get_mut<T: 'static>(&self) -> Option<RefMut<'_, T>> {
        let cell = self.storage.get(&TypeId::of::<T>())?;

        Some(RefMut::map(cell.borrow_mut(), |boxed| {
            boxed.downcast_mut::<T>().unwrap()
        }))
    }
}
