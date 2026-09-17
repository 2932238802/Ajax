use crate::{
    component::{LosHealth, LosHungry, LosMental, LosPosition},
    core::{
        world::{entity, LosEntity, LosWorld},
        save::data::SavePlayer,
    },
};

// 提供一个静态函数
pub struct LosPlayer {}

// 玩家的生成
impl LosPlayer {
    pub fn spawn(world: &mut LosWorld) -> LosEntity {
        let entity = world.spawn();
        world.add_component(
            entity,
            LosHealth {
                l_current: 100.0,
                l_max: 100.0,
            },
        );
        world.add_component(
            entity,
            LosHungry {
                l_current: 100.0,
                l_max: 100.0,
            },
        );
        world.add_component(
            entity,
            LosMental {
                l_current: 100.0,
                l_max: 100.0,
            },
        );
        world.add_component(
            entity,
            // 默认出生在家里
            LosPosition {
                l_position: crate::core::map::terrain::LosTerrain::Home,
            },
        );
        entity
    }

    // 从 保存的数据里 来
    pub fn from_save_data(data: &SavePlayer, world: &mut LosWorld) -> LosEntity {
        let entity: LosEntity = world.spawn();
        world.add_component(
            entity,
            LosHealth {
                l_current: data.l_state.l_cur_health,
                l_max: data.l_state.l_health_max,
            },
        );
        world.add_component(
            entity,
            LosHungry {
                l_current: data.l_state.l_cur_health,
                l_max: data.l_state.l_health_max,
            },
        );
        world.add_component(
            entity,
            LosMental {
                l_current: data.l_state.l_cur_health,
                l_max: data.l_state.l_health_max,
            },
        );
        world.add_component(
            entity,
            // 默认出生在家里
            LosPosition {
                l_position: data.l_position.l_position,
            },
        );
        entity
    }
}
