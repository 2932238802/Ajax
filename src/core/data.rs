use crate::{component::LosPosition, core::terrain::LosTerrain};
use serde::{Deserialize, Serialize};
use std::vec;

// 存储的数据
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveData {
    pub l_player: SavePlayer,
    pub l_map: SaveMap,
    pub l_time: SaveTime,
}

// 个人的信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SavePlayer {
    pub l_entity_id: i32,
    pub l_position: LosPosition,
    pub l_state: SaveState,
}

// 地图信息
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveMap {
    pub l_map: Vec<Vec<LosTerrain>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SaveTime {
    pub l_elasped: u64,
}

// 状态信息 这里一般是个人的信息
// l_cur_health 当前的健康状态
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveState {
    pub l_cur_health: f64,
    pub l_health_max: f64,
    pub l_cur_mental: f64,
    pub l_mental_max: f64,
    pub l_cur_hungry: f64,
    pub l_hungry_max: f64,
}
