use crate::constants::constant_class::GameState;
use crate::core::event::LosEvent;
use crate::core::world::LosEntity;
use crate::core::world::LosWorld;

pub trait UpdateAbleByTime {
    fn update(
        &mut self,
        entity: LosEntity,
        world: &LosWorld,
        elapsed_minutes: f64,
    ) -> Vec<LosEvent>;
}
