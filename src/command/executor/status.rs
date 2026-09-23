use std::{format, print};

use crate::component::{LosHealth, LosHungry, LosMental};
use crate::core::game::LosGame;

#[derive(Debug, Clone, PartialEq)]
pub enum ExeStatusMode {
    ALL,
    HEALTH,
    MENTAL,
    HUNGRY,
    BYENTITY, // 根据 entity id
}

// 展示玩家状态
// status h 状态
// status u 饥饿度
// status m 精神
// status id 跟上一个id
pub fn status(game: &mut LosGame, mode: ExeStatusMode) {
    let health = game
        .l_world
        .get_component::<LosHealth>(game.l_player)
        .unwrap();
    let hungry = game
        .l_world
        .get_component::<LosHungry>(game.l_player)
        .unwrap();
    let mental = game
        .l_world
        .get_component::<LosMental>(game.l_player)
        .unwrap();
    match mode {
        ExeStatusMode::ALL => {
            let s = format!(
                r#"
health: {:.1} 
hungry: {:.1}
mental: {:.1}
"#,
                health.l_current, hungry.l_current, mental.l_current
            );
            println!("{}", s);
        }
        ExeStatusMode::BYENTITY => {}
        ExeStatusMode::HEALTH => {
            let s = format!(
                r#"
health: {:.2} 
"#,
                health.l_current
            );
            println!("{}", s);
        }
        ExeStatusMode::MENTAL => {
            let s = format!(
                r#"
mental: {:.2}
"#,
                mental.l_current
            );
            println!("{}", s);
        }
        ExeStatusMode::HUNGRY => {
            let s = format!(
                r#"
hungry: {:.2}
"#,
                hungry.l_current
            );
            println!("{}", s);
        }
    }
}
