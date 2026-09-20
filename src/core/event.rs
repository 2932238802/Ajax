use crate::core::world::LosEntity;

pub enum LosEvent {
    Death {
        entity: LosEntity,
        cause: DeathEvent,
    },
    OnTheVerge {
        entity: LosEntity,
        cause: OnTheVergeEvent,
    },
}

// 死亡
pub enum DeathEvent {
    Starvation,
    MentalBreak,
    Combat,
}

// 濒临
pub enum OnTheVergeEvent {
    HungryOnTheVerge,
    MentalOnTheVerge,
}
