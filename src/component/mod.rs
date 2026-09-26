// 这个包
// 主要作用就是 一个一个 struct

pub mod eatable;
pub mod health;
pub mod hungry;
pub mod inventory;
pub mod item_id;
pub mod mental;
pub mod position;
pub mod carriable;

pub use health::LosHealth;
pub use hungry::LosHungry;
pub use mental::LosMental;
pub use position::LosPosition;
