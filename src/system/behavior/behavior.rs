use crate::{
    component::apply_able::ApplyWays,
    ecs::{LosEntity, LosWorld},
    game::event::LosEvent,
};

// 实体行为
// 一个实体可以挂载多个行为；行为只需实现自己关心的钩子，其余用默认空实现
pub trait LosBehavior {
    // 随时间推进触发（elapsed 单位：游戏分钟）
    fn on_time(&self, _world: &mut LosWorld, _entity: LosEntity, _elapsed: f64) -> Vec<LosEvent> {
        Vec::new()
    }

    // 动作交互统一抽象
    // 返回 Option<Vec<LosEvent>>:
    // - None: 该 Behavior 不响应该 way 操作
    // - Some(events): 成功响应并返回产生的事件
    fn on_apply(
        &self,
        _world: &mut LosWorld,
        _actor: LosEntity,
        _target: LosEntity,
        _way: ApplyWays,
    ) -> Option<Vec<LosEvent>> {
        None
    }
}
