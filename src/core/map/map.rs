use std::{print, println};

use crate::core::{map::terrain::LosTerrain, save::data::SaveMap};
use rand::{rngs::ThreadRng, Rng, RngExt};

// LosMap
pub struct LosMap {
    l_width: usize,
    l_height: usize,
    l_map: Vec<Vec<LosTerrain>>,
}

// 接口
impl LosMap {
    pub fn new(width: usize, height: usize) -> Self {
        let mut los_map = Self {
            l_height: height,
            l_width: width,
            l_map: vec![vec![LosTerrain::None; width]; height],
        };
        // 枚举 -> 地形
        let terrains = vec![
            LosTerrain::Home,
            LosTerrain::Forest,
            LosTerrain::Plains,
            LosTerrain::Mountain,
            LosTerrain::River,
            LosTerrain::Swamp,
            LosTerrain::Desert,
            LosTerrain::Cave,
            LosTerrain::Beach,
            LosTerrain::Lake,
        ];

        // 生成 0.9 的随机数
        let mut rng = rand::rng();

        for terr in terrains {
            loop {
                let x = rng.random_range(0..width);
                let y = rng.random_range(0..height);
                if los_map.l_map[x][y] == LosTerrain::None {
                    los_map.l_map[x][y] = terr;
                    break;
                }
            }
        }
        los_map
    }

    pub fn get_data(&self) -> &[Vec<LosTerrain>] {
        &self.l_map
    }

    // 克隆数据
    pub fn from_save_data(map: &SaveMap) -> Self {
        let map: Self = Self {
            l_map: map.l_map.clone(),
            l_width: map.l_map.len(),
            l_height: map.l_map[0].len(),
        };
        map
    }

    // 计算两个位置的沟谷位置
    pub fn distance(&self, from: (usize, usize), to: (usize, usize)) -> f64 {
        let dx = from.0 as f64 - to.0 as f64;
        let dy = from.0 as f64 - to.0 as f64;
        (dx * dx + dy * dy).sqrt()
    }

    // 打印地图
    pub fn show(&self) {
        // 默认都是 (行，列)
        for i in 0..self.l_width {
            print!("{}  ", i + 1);
            for j in 0..self.l_height {
                print!("{}  ", self.l_map[i][j]);
            }
            println!();
            println!();
        }
    }
}
