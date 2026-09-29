use crate::{
    component::{inventory::LosInventory, item_id::LosItemId},
    game::LosGame,
};

// 玩家指令 展示 背包
pub fn bag(game: &LosGame) {
    let inventory_option = game.l_world.get_component::<LosInventory>(game.l_player);
    if let Some(inventory) = inventory_option {
        inventory.show_items("玩家");
    }
}
