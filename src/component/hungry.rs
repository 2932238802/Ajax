use serde::{Deserialize, Serialize};

use crate::component::update_able::UpdateAbleByTime;
use crate::constants::constant_number::HUNGRY_MUTLY;
use crate::core::event::{LosEvent, OnTheVergeEvent};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LosHungry {
    pub l_current: f64,
    pub l_max: f64,
}