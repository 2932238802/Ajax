use std::io::{self, Write};

use crate::{
    command::command::UseTarget,
    common::los_type::LosType,
    component::{
        apply_able::ApplyWays, carriable::LosCarriable, inventory::LosInventory, item_id::LosItemId,
    },
    ecs::LosEntity,
    game::LosGame,
    system::LosUpdate,
};

pub fn apply(game: &mut LosGame, target: UseTarget, pre_way: Option<ApplyWays>) {
    let (item_id, target_entity) = {
        let inv = match game.l_world.get_component::<LosInventory>(game.l_player) {
            Some(inv) => inv,
            None => {
                println!("! 玩家缺少背包组件");
                return;
            }
        };

        match target {
            UseTarget::ById(id) => {
                let ent = LosEntity { l_id: id };
                match inv.has(ent) {
                    Some(id) => (id, ent),
                    None => {
                        println!("! 玩家身上没有 ID 为 {} 的物品", id);
                        return;
                    }
                }
            }
            UseTarget::ByName(name) => {
                let kind = LosType::to_item(&name);
                let id = LosItemId(kind);
                // peek 是拿到最后一个
                match inv.peek(&id) {
                    Some(ent) => (id, ent),
                    None => {
                        println!("! 玩家身上没有 {}", kind.get_name());
                        return;
                    }
                }
            }
        }
    };
    let available_ways = item_id.0.get_apply_ways();
    if available_ways.is_empty() {
        println!("! {} 无法进行任何操作", item_id.0.get_name());
        return;
    }
    let chosen_way = if let Some(w) = pre_way {
        if !available_ways.contains(&w) {
            println!("! {} 无法进行「{}」操作", item_id.0.get_name(), w.name());
            return;
        }
        w
    } else {
        println!("已在身上找到 {}", item_id.0.get_name());
        println!("以下是具体可使用的方式 >>>");
        println!(" 0 -> 取消使用");
        for (i, w) in available_ways.iter().enumerate() {
            println!(" {} -> {}", i + 1, w.name());
        }

        loop {
            print!("确定使用方式（输入对应的数字）: ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("! 输入读取失败，请重新输入");
                continue;
            }

            match input.trim().parse::<usize>() {
                Ok(0) => {
                    println!("-> 已取消使用。");
                    return;
                }
                Ok(n) if n <= available_ways.len() => {
                    break available_ways[n - 1];
                }
                _ => println!(
                    "! 输入无效，请输入 0 ~ {} 之间的数字！",
                    available_ways.len()
                ),
            }
        }
    };

    let events = match LosUpdate::apply(
        &game.l_register,
        &mut game.l_world,
        game.l_player,
        target_entity,
        chosen_way,
    ) {
        Ok(evs) => evs,
        Err(msg) => {
            println!("! 使用失败: {}", msg);
            return;
        }
    };

    match chosen_way {
        ApplyWays::EAT => {
            let size = game
                .l_world
                .get_component::<LosCarriable>(target_entity)
                .map(|c| c.l_size)
                .unwrap_or(0.0);

            if let Some(inv) = game
                .l_world
                .get_component_mut::<LosInventory>(game.l_player)
            {
                let _ = inv.try_take_one(&item_id);
                inv.release_size(size);
            }

            game.l_register.remove_entity(target_entity);
            game.l_world.despawn(target_entity);

            println!("✓ 你吃掉了 {}", item_id.0.get_name());
            for ev in events {
                println!("{}", ev);
            }
        }
        ApplyWays::GIVE => {
            println!("给予功能暂未实现...");
        }
    }
}
