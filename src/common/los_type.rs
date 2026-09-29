use crate::entity::item::LosItem;

pub struct LosType;
impl LosType {
    pub fn to_integer(s: &str) -> i64 {
        s.parse::<i64>().unwrap()
    }

    pub fn is_integer(s: &str) -> bool {
        s.parse::<i64>().is_ok()
    }

    // 如果是 none is_none 就是 true ! 取反就是 false
    // 所以是 物品 就是 true
    // 不是物品 就是 false
    pub fn is_item(s: &str) -> bool {
        !LosItem::from_name(s).is_none()
    }

    pub fn to_item(s: &str) -> LosItem {
        LosItem::from_name(s).unwrap()
    }
}
