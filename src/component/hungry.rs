use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LosHungry {
    pub l_current: f64,
    pub l_max: f64,
    // 每分钟饥饿下降量
    pub l_decay_rate: f64,
    // 低于此值视为「饥饿濒死」
    pub l_starving_threshold: f64,
}

impl Default for LosHungry {
    fn default() -> Self {
        Self {
            l_current: 100.0,
            l_max: 100.0,
            l_decay_rate: 0.23,
            l_starving_threshold: 10.0,
        }
    }
}
