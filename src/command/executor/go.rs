use crate::{
    component::LosPosition,
    core::{game::LosGame, map::LosMap},
};

// 到达 一个 位置
pub fn go(game: &mut LosGame, to_place: &str) {
    let pos_position = game.l_world.get_component_mut::<LosPosition>(game.l_player);
    match pos_position {
        Some(pos) => if pos.l_position.to_string() != to_place {},
        None => {
            eprintln!("内部发生错误 player 没有 LosPosition 组件");
        }
    }
}
