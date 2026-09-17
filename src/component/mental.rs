
use serde::{Deserialize, Serialize};

#[derive(
    Debug, 
    Clone, 
    PartialEq,
    Deserialize, 
    Serialize)]
pub struct LosMental {
    pub l_current: f64,
    pub l_max: f64,
}
