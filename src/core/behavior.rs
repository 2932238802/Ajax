use crate::core::{
    event::LosEvent,
    world::{LosEntity, LosWorld},
};

// 实体行为
// 一个实体可以挂载多个行为；行为只需实现自己关心的钩子，其余用默认空实现
pub trait LosBehavior {
    // 随时间推进触发（elapsed 单位：游戏分钟）
    fn on_time(&self, _world: &mut LosWorld, _entity: LosEntity, _elapsed: f64) -> Vec<LosEvent> {
        Vec::new()
    }

    // 被 使用 时触发
    // user   : 使用者
    // target : 被使用者（通常是自己）
    fn on_use(&self, _world: &mut LosWorld, _user: LosEntity, _target: LosEntity) -> Vec<LosEvent> {
        Vec::new()
    }
}
