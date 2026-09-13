use crate::core::map::terrain::LosTerrain;
use serde::{Deserialize, Serialize};

// 位置
#[derive(
    Debug, 
    Clone, 
    PartialEq,
    Deserialize, 
    Serialize)]
pub struct LosPosition {
    pub l_position: LosTerrain,
}
