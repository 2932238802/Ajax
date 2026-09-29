use crate::{
    component::LosMental,
    ecs::{LosEntity, LosWorld},
    game::{event::OnTheVergeEvent, LosEvent},
    system::behavior::behavior::LosBehavior,
};

// 精神：随时间下降
pub struct MentalBehavior;
impl LosBehavior for MentalBehavior {
    fn on_time(&self, world: &mut LosWorld, entity: LosEntity, elapsed: f64) -> Vec<LosEvent> {
        let mut events = Vec::new();
        if let Some(mental) = world.get_component_mut::<LosMental>(entity) {
            mental.l_current = (mental.l_current - mental.l_decay_rate * elapsed).max(0.0);
            if mental.l_current <= mental.l_verge_threshold {
                events.push(LosEvent::OnTheVerge {
                    entity,
                    cause: OnTheVergeEvent::MentalOnTheVerge,
                });
            }
        }
        events
    }
}
