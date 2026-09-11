
// 位置
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LosPositionEnum {
    HOME,
    FOREST1,
    FOREST2,
    RIVERO1
}



// 位置
#[derive(Debug, Clone, PartialEq)]
pub struct LosPosition {
    pub l_position: LosPositionEnum,
}
