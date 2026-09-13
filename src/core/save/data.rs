use std::vec;

use serde::{Deserialize, Serialize};

use crate::{component::LosPosition, core::map::terrain::LosTerrain};

// 存储的数据
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveData 
{
    pub l_player: SavePlayer,
    pub l_map: SaveMap
}

// 个人的信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SavePlayer {
    pub l_entity_id: u32,
    pub l_position: LosPosition,
    pub l_state: SaveState,
}

// 地图信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveMap {
    pub l_map: Vec<Vec<LosTerrain>>,
}

// 状态信息 这里一般是个人的信息
// l_cur_health 当前的健康状态
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveState {
    pub l_cur_health: f64,
    pub l_cur_max_health: f64
}
