use crate::{
    app::AppResources,
    ecs::{ECS, entity_manager::Entity},
};

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

    fn start(&mut self, entities: &mut [Entity], ecs: &mut ECS, res: &mut AppResources);

    fn update(&mut self, entities: &mut [Entity], ecs: &mut ECS, res: &mut AppResources);

    fn fixed_update(&mut self, entities: &mut [Entity]);
}

pub struct System {
    pub entities: Vec<Entity>,
    pub system: Box<dyn SystemTrait>,
}
