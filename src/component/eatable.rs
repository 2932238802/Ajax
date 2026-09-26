use serde::{Deserialize, Serialize};

// 能够吃
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LosEdible {
    pub l_hunger: f64,
    pub l_mental: f64,
    pub l_health: f64,
}
