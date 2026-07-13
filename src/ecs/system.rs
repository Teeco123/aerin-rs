use crate::ecs::entity_manager::Entity;

pub type SystemSignature = u32;

pub struct System {
    entities: Vec<Entity>,
}

pub trait SystemTrait {
    fn type_of() -> &'static str;
}
