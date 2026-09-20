use crate::component::update_able::UpdateAbleByTime;
use crate::constants::constant_number::MENTAL_MUTLY;
use crate::core::event::{LosEvent, OnTheVergeEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LosMental {
    pub l_current: f64,
    pub l_max: f64,
}

// 精神状态
impl UpdateAbleByTime for LosMental {
    fn update(
        &mut self,
        entity: crate::core::world::LosEntity,
        _: &crate::core::world::LosWorld,
        elapsed_minutes: f64,
    ) -> Vec<crate::core::event::LosEvent> {
        let mut contain = Vec::new();
        self.l_current -= MENTAL_MUTLY * elapsed_minutes as f64;
        if self.l_current <= 0.0 {
            self.l_current = 0.0;
        }
        if self.l_current <= 40.0 {
            contain.push(LosEvent::OnTheVerge {
                entity: entity,
                cause: OnTheVergeEvent::MentalOnTheVerge
            });
        }
        contain
    }
}
