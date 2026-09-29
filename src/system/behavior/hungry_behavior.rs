use crate::{
    component::LosHungry,
    ecs::{LosEntity, LosWorld},
    game::{event::OnTheVergeEvent, LosEvent},
    system::behavior::behavior::LosBehavior,
};

pub struct HungerBehavior;

impl LosBehavior for HungerBehavior {
    fn on_time(&self, world: &mut LosWorld, entity: LosEntity, elapsed: f64) -> Vec<LosEvent> {
        let mut events = Vec::new();
        if let Some(hungry) = world.get_component_mut::<LosHungry>(entity) {
            hungry.l_current = (hungry.l_current - hungry.l_decay_rate * elapsed).max(0.0);
            if hungry.l_current <= hungry.l_starving_threshold {
                events.push(LosEvent::OnTheVerge {
                    entity,
                    cause: OnTheVergeEvent::HungryOnTheVerge,
                });
            }
        }
        events
    }
}
