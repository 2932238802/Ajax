use crate::core::{game::LosGame, map::map::LosMap};

// 展示地图
pub fn show_map(game: &mut LosGame)
{
    let map: &LosMap = &game.l_world.l_map;
    map.show();
}