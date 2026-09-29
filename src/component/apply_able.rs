#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApplyWays {
    EAT,  // 吃
    GIVE, // 给予 传递 给
}

impl ApplyWays {
    pub fn name(&self) -> &str {
        match self {
            ApplyWays::EAT => "can eat!",
            ApplyWays::GIVE => "can give!",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LosApplyAble {}
