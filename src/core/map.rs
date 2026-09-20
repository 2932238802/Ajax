use std::{print, println};

use crate::{
    component::LosPosition,
    constants::constant_str::{
        LT_BEACH, LT_CAVE, LT_DESERT, LT_FOREST, LT_HOME, LT_LAKE, LT_MOUNTAIN, LT_PLAINS,
        LT_RIVER, LT_SWAMP, MAP_TITLE,
    },
    core::{data::SaveMap, terrain::LosTerrain},
};
use colored::*;
use rand::{Rng, RngExt};

pub struct LosMap {
    l_rows: usize,
    l_cols: usize,
    l_map: Vec<Vec<LosTerrain>>,
}

impl LosMap {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut los_map = Self {
            l_rows: rows,
            l_cols: cols,
            l_map: vec![vec![LosTerrain::None; cols]; rows],
        };

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

        let mut rng = rand::rng();
        for terr in terrains {
            loop {
                let row = rng.random_range(0..rows);
                let col = rng.random_range(0..cols);
                if los_map.l_map[row][col] == LosTerrain::None {
                    los_map.l_map[row][col] = terr;
                    break;
                }
            }
        }
        los_map
    }

    pub fn from_save_data(map: &SaveMap) -> Self {
        let l_map = map.l_map.clone();
        let l_rows = l_map.len();
        let l_cols = if l_rows > 0 { l_map[0].len() } else { 0 };

        Self {
            l_rows,
            l_cols,
            l_map,
        }
    }

    // 获取数据
    pub fn get_data(&self) -> &[Vec<LosTerrain>] {
        &self.l_map
    }

    pub fn distance(&self, from: (usize, usize), to: (usize, usize)) -> f64 {
        let dx = from.0 as f64 - to.0 as f64;
        let dy = from.1 as f64 - to.1 as f64;
        (dx * dx + dy * dy).sqrt()
    }

    // 展示
    pub fn show(&self, position: &LosPosition) {
        println!();
        let border_width = 4 + self.l_cols * 5;
        let border = "═".repeat(border_width);
        // 顶部边框
        println!("{}", format!("╔{}╗", border).cyan());

        // 标题居中
        let title = MAP_TITLE;
        let title_display_width = MAP_TITLE.len();
        let padding_left = (border_width - title_display_width) / 2;
        let padding_right = border_width - title_display_width - padding_left;
        println!(
            "{}{}{}{}{}",
            "║".cyan(),
            " ".repeat(padding_left),
            title.bright_cyan().bold(),
            " ".repeat(padding_right),
            "║".cyan()
        );

        println!("{}", format!("╠{}╣", border).cyan());

        // 列号行
        print!("{}", "║".cyan());
        print!("     "); // 左侧空白（行号位置）

        for col in 0..self.l_cols {
            if col == self.l_cols - 1 {
                // 最后一列不加尾随空格
                print!(" {:^3}", col.to_string().bright_white());
            } else {
                print!(" {:^3} ", col.to_string().bright_white());
            }
        }

        println!("{} ", "║".cyan());

        println!("{}", format!("╠{}╣", border).cyan());

        // 地图内容
        for row in 0..self.l_rows {
            print!("{}", "║".cyan());
            print!(" {:>2} ", row.to_string().bright_white());

            for col in 0..self.l_cols {
                let terrain = &self.l_map[row][col];
                if *terrain == position.l_position {
                    let text = format!("[{}]", self.get_terrain_char(terrain));
                    print!(" {:^3} ", text.green().bold());
                } else {
                    print!(" {:^3} ", self.colorize_terrain(terrain));
                }
            }

            println!("{} ", "║".cyan());
        }

        // 底部边框
        println!("{}", format!("╚{}╝", border).cyan());
    }

    fn get_terrain_char(&self, terrain: &LosTerrain) -> &str {
        match terrain {
            LosTerrain::Home => LT_HOME,         // 家
            LosTerrain::Forest => LT_FOREST,     // 林
            LosTerrain::Plains => LT_PLAINS,     // 平原
            LosTerrain::Mountain => LT_MOUNTAIN, // 山
            LosTerrain::River => LT_RIVER,       // 河流
            LosTerrain::Swamp => LT_SWAMP,       // 沼泽
            LosTerrain::Desert => LT_DESERT,     // 沙漠
            LosTerrain::Cave => LT_CAVE,         // 洞
            LosTerrain::Beach => LT_BEACH,       // 沙滩
            LosTerrain::Lake => LT_LAKE,         // 湖
            LosTerrain::None => "·",
        }
    }

    // 字符串 到 地形
    pub fn str_to_terrain(place: &str) -> LosTerrain {
        match place {
            LT_HOME => LosTerrain::Home,         // 家
            LT_FOREST => LosTerrain::Forest,     // 林
            LT_PLAINS => LosTerrain::Plains,     // 平原
            LT_MOUNTAIN => LosTerrain::Mountain, // 山
            LT_RIVER => LosTerrain::River,       // 河流
            LT_SWAMP => LosTerrain::Swamp,       // 沼泽
            LT_DESERT => LosTerrain::Desert,     // 沙漠
            LT_CAVE => LosTerrain::Cave,         // 洞
            LT_BEACH => LosTerrain::Beach,       // 沙滩
            LT_LAKE => LosTerrain::Lake,         // 湖
            _ => LosTerrain::None,
        }
    }

    pub fn isvalid(place: &str) -> bool {
        [
            LT_BEACH,
            LT_CAVE,
            LT_DESERT,
            LT_FOREST,
            LT_HOME,
            LT_LAKE,
            LT_MOUNTAIN,
            LT_PLAINS,
            LT_RIVER,
            LT_SWAMP,
            MAP_TITLE,
        ]
        .contains(&place)
    }

    fn colorize_terrain(&self, terrain: &LosTerrain) -> ColoredString {
        let ch = self.get_terrain_char(terrain);
        match terrain {
            LosTerrain::Home => ch.yellow().bold(),
            LosTerrain::Forest => ch.green(),
            LosTerrain::Plains => ch.bright_green(),
            LosTerrain::Mountain => ch.bright_white().bold(),
            LosTerrain::River => ch.bright_blue(),
            LosTerrain::Swamp => ch.truecolor(100, 200, 150),
            LosTerrain::Desert => ch.bright_yellow(),
            LosTerrain::Cave => ch.truecolor(150, 150, 150),
            LosTerrain::Beach => ch.bright_cyan(),
            LosTerrain::Lake => ch.blue(),
            LosTerrain::None => ch.bright_black(),
        }
    }
}
