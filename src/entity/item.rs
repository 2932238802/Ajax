use serde::{Deserialize, Serialize};

use crate::component::apply_able::ApplyWays;

#[derive(Debug, Hash, Clone, PartialEq, Eq, Copy, Serialize, Deserialize)]
pub enum LosItem {
    Apple, // 苹果
    Berry, // 浆果
    None,
}

impl LosItem {
    pub fn get_name(&self) -> &str {
        match self {
            LosItem::Apple => "苹果",
            LosItem::Berry => "浆果",
            LosItem::None => "空",
        }
    }

    pub fn get_describe(&self) -> &str {
        match self {
            LosItem::Apple => "能够恢复一定的生命值，是一个不错的充饥水果",
            LosItem::Berry => "普通的浆果，可以补充体力",
            LosItem::None => "空 空 空",
        }
    }

    // 从 名字 回到 类型
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "apple" | "苹果" => Some(LosItem::Apple),
            "berry" | "浆果" => Some(LosItem::Berry),
            _ => None,
        }
    }

    pub fn can_apply(&self, way: ApplyWays) -> bool {
        match self {
            LosItem::Apple => matches!(way, ApplyWays::EAT | ApplyWays::GIVE),
            LosItem::Berry => matches!(way, ApplyWays::EAT),
            LosItem::None => false,
        }
    }

    pub fn get_apply_ways(&self) -> &'static [ApplyWays] {
        match self {
            LosItem::Apple => &[ApplyWays::EAT, ApplyWays::GIVE],
            LosItem::Berry => &[ApplyWays::EAT],
            LosItem::None => &[],
        }
    }
}
