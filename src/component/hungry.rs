use serde::{Deserialize, Serialize};

use crate::component::update_able::UpdateAbleByTime;
use crate::constants::constant_number::HUNGRY_MUTLY;
use crate::core::event::{LosEvent, OnTheVergeEvent};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LosHungry {
    pub l_current: f64,
    pub l_max: f64,
}

impl UpdateAbleByTime for LosHungry {
    fn update(
        &mut self,
        entity: crate::core::world::LosEntity,
        _: &crate::core::world::LosWorld,
        elapsed_minutes: f64,
    ) -> Vec<crate::core::event::LosEvent> {
        let mut contain = Vec::new();
        self.l_current -= HUNGRY_MUTLY * elapsed_minutes as f64;
        if self.l_current <= 0.0 {
            self.l_current = 0.0;
        }
        if self.l_current <= 10.0 {
            contain.push(LosEvent::OnTheVerge {
                entity: entity,
                cause: OnTheVergeEvent::HungryOnTheVerge,
            });
        }
        contain
    }
}
