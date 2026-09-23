use crate::constants::constant_number::MENTAL_MUTLY;
use crate::core::event::{LosEvent, OnTheVergeEvent};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LosMental {
    pub l_current: f64,
    pub l_max: f64,
}


