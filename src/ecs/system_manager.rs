use std::collections::HashMap;

use crate::ecs::system::{System, SystemSignature, SystemTrait};

pub struct SystemManager {
    systems: HashMap<&'static str, System>,
    signatures: HashMap<&'static str, SystemSignature>,
}

impl SystemManager {
    pub fn new() -> Self {
        Self {
            systems: HashMap::new(),
            signatures: HashMap::new(),
        }
    }
    pub fn register_system<T: SystemTrait + 'static>(&mut self) {
        unimplemented!()
    }
}
