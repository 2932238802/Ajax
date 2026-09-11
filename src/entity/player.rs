use crate::{
    component::{position::LosPositionEnum, LosHealth, LosPosition},
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
                l_current: 100,
                l_max: 100,
            },
        );
        world.add_component(
            entity,
            LosPosition {
                l_position: LosPositionEnum::HOME,
            },
        );
        entity
    }
}
