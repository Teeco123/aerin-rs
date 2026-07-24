use crate::ecs::entity_manager::Entity;

pub type SystemSignature = u32;

pub trait SystemTrait {
    fn new() -> Self
    where
        Self: Sized + Default,
    {
        Self::default()
    }

    fn type_of() -> &'static str
    where
        Self: Sized;

    fn update(&mut self, entities: &mut [Entity]);

    fn fixed_update(&mut self, entities: &mut [Entity]);
}

pub struct System {
    pub entities: Vec<Entity>,
    pub system: Box<dyn SystemTrait>,
}
