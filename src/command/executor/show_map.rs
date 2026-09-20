use crate::{
    component::LosPosition,
    core::{game::LosGame, map::LosMap},
};

// 展示地图
// 就是拿到 LosGame
pub fn show_map(game: &mut LosGame) {
    let position_option = game.l_world.get_component::<LosPosition>(game.l_player);
    match position_option {
        Some(position) => {
            let map: &LosMap = &game.l_world.l_map;
            map.show(position);
        }
        None => {
            eprintln!("player 没有 position 这个组件! 内部错误");
        }
    }
}
