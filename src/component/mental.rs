use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LosMental {
    pub l_current: f64,
    pub l_max: f64,
    // 每分钟精神下降量
    pub l_decay_rate: f64,
    // 低于此值视为「精神崩溃」（开始扣血）
    pub l_broken_threshold: f64,
    // 低于此值弹出濒死警告
    pub l_verge_threshold: f64,
}

// 影响一次
pub struct LosAlterMentalOnce {
    pub l_min: f64,
    pub l_max: f64,
}

// 持续影响
pub struct LosAlterMentalConsistent {
    pub l_min: f64,
    pub l_max: f64,
    pub l_times: usize,
    pub l_interval: f64,
}

impl Default for LosMental {
    fn default() -> Self {
        Self {
            l_current: 90.0,
            l_max: 100.0,
            l_decay_rate: 0.001,
            l_broken_threshold: 15.0,
            l_verge_threshold: 40.0,
        }
    }
}
