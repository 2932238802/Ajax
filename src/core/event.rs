use std::fmt::{Display, Formatter, Result};

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

impl Display for LosEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            LosEvent::Death { cause, .. } => write!(f, "{}", cause),
            LosEvent::OnTheVerge { cause, .. } => write!(f, "{}", cause),
        }
    }
}

impl Display for DeathEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let text = match self {
            DeathEvent::Starvation => "你饿死了",
            DeathEvent::MentalBreak => "你的精神崩溃了",
            DeathEvent::Combat => "你在战斗中倒下了",
        };
        write!(f, "{}", text)
    }
}

impl Display for OnTheVergeEvent {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let text = match self {
            OnTheVergeEvent::HungryOnTheVerge => "你饿得头晕眼花",
            OnTheVergeEvent::MentalOnTheVerge => "你的精神濒临崩溃",
        };
        write!(f, "{}", text)
    }
}
