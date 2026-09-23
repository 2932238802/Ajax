use serde::{Deserialize, Serialize};

use crate::{
    component::{LosHungry, LosMental},
    constants::{
        constant_class::GameState,
        constant_number::{HUNGRY_AFFECT_HEALTH_MULTY, MENTAL_AFFECT_HEALTH_MULTY},
    },
    core::event::{LosEvent, OnTheVergeEvent::HungryOnTheVerge},
};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LosHealth {
    pub l_current: f64,
    pub l_max: f64,
}

// impl UpdateAbleByTime for LosHealth {
//     fn update(
//         &mut self,
//         entity: crate::core::world::LosEntity,
//         world: &crate::core::world::LosWorld,
//         elapsed_minutes: f64,
//     ) -> Vec<LosEvent> {
//         let mut contain = Vec::new();
//         if let Some(hungry) = world.get_component::<LosHungry>(entity) {
//             if hungry.l_current < 10.0 {
//                 let damage: f64 = HUNGRY_AFFECT_HEALTH_MULTY * elapsed_minutes as f64;
//                 self.l_current -= damage;
//                 if self.l_current <= 0.0 {
//                     // 如何 扔出 事件 或者说 通知 死亡?
//                     contain.push(LosEvent::Death {
//                         entity,
//                         cause: crate::core::event::DeathEvent::Starvation,
//                     });
//                 }
//             }
//         }
//         if let Some(mental) = world.get_component::<LosMental>(entity) {
//             if mental.l_current < 15.0 {
//                 let damage: f64 = MENTAL_AFFECT_HEALTH_MULTY * elapsed_minutes as f64;
//                 self.l_current -= damage;
//                 if self.l_current <= 0.0 {
//                     // 如何 扔出 事件 或者说 通知 死亡?
//                     contain.push(LosEvent::Death {
//                         entity,
//                         cause: crate::core::event::DeathEvent::MentalBreak,
//                     });
//                 }
//             }
//         }
//         contain
//     }
// }
