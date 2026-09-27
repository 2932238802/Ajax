use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LosHealth {
    pub l_current: f64,
    pub l_max: f64,
    // 饥饿濒死时每分钟扣血
    pub l_hunger_damage_rate: f64,
    // 精神崩溃时每分钟扣血
    pub l_mental_damage_rate: f64,
}

impl Default for LosHealth {
    fn default() -> Self {
        Self {
            l_current: 100.0,
            l_max: 100.0,
            l_hunger_damage_rate: 0.4,
            l_mental_damage_rate: 0.3,
        }
    }
}
