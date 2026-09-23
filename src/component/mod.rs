// 这个包
// 主要作用就是 一个一个 struct

pub mod health;
pub mod position;
pub mod hungry;
pub mod mental;

pub use health::LosHealth;
pub use position::LosPosition;
pub use hungry::LosHungry;
pub use mental::LosMental;