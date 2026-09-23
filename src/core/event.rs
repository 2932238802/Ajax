use crate::core::world::LosEntity;

#[derive(Debug)]
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
#[derive(Debug)]
pub enum DeathEvent {
    Starvation,
    MentalBreak,
    Combat,
}

// 濒临
#[derive(Debug)]
pub enum OnTheVergeEvent {
    HungryOnTheVerge,
    MentalOnTheVerge,
}
