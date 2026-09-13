use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};

// 地形枚举
#[derive(
    Debug, 
    Clone, 
    Copy, 
    PartialEq, 
    Eq,
    Deserialize,
    Serialize
)]
pub enum LosTerrain {
    Home,     // 家
    Forest,   // 森林
    Plains,   // 平原
    Mountain, // 山地
    River,    // 河流
    Swamp,    // 沼泽
    Desert,   // 沙漠
    Cave,     // 洞穴
    Beach,    // 海滩
    Lake,     // 湖泊
    None,     // 没有地形 就是 无触发
}

impl Display for LosTerrain {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LosTerrain::Home => write!(f, "家"),
            LosTerrain::Forest => write!(f, "森林"),
            LosTerrain::Plains => write!(f, "平原"),
            LosTerrain::Mountain => write!(f, "山地"),
            LosTerrain::River => write!(f, "河流"),
            LosTerrain::Swamp => write!(f, "沼泽"),
            LosTerrain::Desert => write!(f, "沙漠"),
            LosTerrain::Cave => write!(f, "洞穴"),
            LosTerrain::Beach => write!(f, "沙滩"),
            LosTerrain::Lake => write!(f, "湖泊"),
            LosTerrain::None => write!(f, "    "),
        }
    }
}

// 接口
impl LosTerrain {
    // 获取 名字
    pub fn get_name(&self) -> &str {
        match self {
            LosTerrain::Home => "家",
            LosTerrain::Forest => "森林",
            LosTerrain::Plains => "平原",
            LosTerrain::Mountain => "山地",
            LosTerrain::River => "河流",
            LosTerrain::Swamp => "沼泽",
            LosTerrain::Desert => "沙漠",
            LosTerrain::Cave => "洞穴",
            LosTerrain::Beach => "海滩",
            LosTerrain::Lake => "湖泊",
            LosTerrain::None => "空",
        }
    }

    // 获取描述
    pub fn get_description(&self) -> &str {
        match self {
            LosTerrain::Home => "你的家，安全温暖",
            LosTerrain::Forest => "茂密的森林，可能有野兽出没",
            LosTerrain::Plains => "开阔的平原，视野良好",
            LosTerrain::Mountain => "险峻的山地，难以通行",
            LosTerrain::River => "湍急的河流，可以取水",
            LosTerrain::Swamp => "危险的沼泽，容易陷入",
            LosTerrain::Desert => "干燥的沙漠，极度缺水",
            LosTerrain::Cave => "黑暗的洞穴，深不见底",
            LosTerrain::Beach => "宁静的海滩",
            LosTerrain::Lake => "清澈的湖泊，水源充足",
            LosTerrain::None => "如履平地",
        }
    }
}
