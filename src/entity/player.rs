use crate::{
    component::{LosHealth, LosPosition},
    core::ecs::{LosEntity, LosWorld},
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
            // 默认出生在家里
            LosPosition {
                l_position: crate::core::map::terrain::LosTerrain::Home,
            },
        );
        entity
    }
}
