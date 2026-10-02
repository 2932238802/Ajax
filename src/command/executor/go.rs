use crate::{
    component::LosPosition,
    game::{LosGame, LosTime},
    world::{LosMap, LosTerrain},
};

// 到达一个位置
pub fn go(game: &mut LosGame, to_place: &str) {
    // 先只读取当前位置，复制出 LosTerrain，避免后面长期持有 world 的可变借用
    let current_terrain = match game.l_world.get_component::<LosPosition>(game.l_player) {
        Some(pos) => pos.l_position,
        None => {
            eprintln!("内部发生错误：player 没有 LosPosition 组件");
            return;
        }
    };
    if current_terrain.to_string() == to_place {
        println!("你已经在这个位置了");
        return;
    }
    let target_terrain = LosMap::str_to_terrain(to_place);
    if target_terrain == LosTerrain::None {
        eprintln!("无效的目标位置：{}", to_place);
        return;
    }
    let from = game.l_world.l_map.get_pos(&current_terrain);
    let to = game.l_world.l_map.get_pos(&target_terrain);
    let cost_minutes = game.l_world.l_map.distance(from, to).ceil() as u64;
    let action_name = format!("正在前往 {}", target_terrain);
    game.l_time.do_action(&action_name, cost_minutes);
    // 动画完成后，真正更新玩家位置
    if let Some(pos) = game.l_world.get_component_mut::<LosPosition>(game.l_player) {
        pos.l_position = target_terrain;
    }
}
