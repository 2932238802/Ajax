use crate::{
    component::{LosHealth, LosHungry, LosMental},
    ecs::{LosEntity, LosWorld},
    game::{event::DeathEvent, LosEvent},
    system::behavior::behavior::LosBehavior,
};

pub struct HealthBehavior;
impl LosBehavior for HealthBehavior {
    fn on_time(&self, world: &mut LosWorld, entity: LosEntity, elapsed: f64) -> Vec<LosEvent> {
        let mut events = Vec::new();
        let starving = world
            .get_component::<LosHungry>(entity)
            .map(|h| h.l_current < h.l_starving_threshold)
            .unwrap_or(false);
        let broken = world
            .get_component::<LosMental>(entity)
            .map(|m| m.l_current < m.l_broken_threshold)
            .unwrap_or(false);
        if let Some(health) = world.get_component_mut::<LosHealth>(entity) {
            if starving {
                health.l_current -= health.l_hunger_damage_rate * elapsed;
            }
            if broken {
                health.l_current -= health.l_mental_damage_rate * elapsed;
            }
            if health.l_current <= 0.0 {
                events.push(LosEvent::Death {
                    entity,
                    cause: DeathEvent::Starvation,
                });
            }
        }
        events
    }
}
