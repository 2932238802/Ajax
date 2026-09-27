#[derive(Debug, Hash, Clone, PartialEq, Eq, Copy)]
pub enum LosItem {
    Apple, // 苹果
    Berry, // 浆果
}

impl LosItem {
    pub fn get_name(&self) -> &str {
        match self {
            LosItem::Apple => "苹果",
            LosItem::Berry => "浆果",
        }
    }

    pub fn get_describe(&self) -> &str {
        match self {
            LosItem::Apple => "能够恢复一定的生命值，是一个不错的充饥水果",
            LosItem::Berry => "普通的浆果，可以补充体力",
        }
    }

    // 从 名字 回到 类型
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "apple" => Some(LosItem::Apple),
            "berry" => Some(LosItem::Berry),
            _ => None,
        }
    }
}
