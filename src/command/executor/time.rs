use std::println;

use crate::core::game::LosGame;

// 显示时间
pub fn time(game: &LosGame) {
    println!("{}", &game.l_time);
}
