use crate::{
    component::apply_able::ApplyWays,
    ecs::{LosEntity, LosWorld},
    game::event::LosEvent,
    system::register::LosFuncRegister,
};

pub struct LosUpdate;

impl LosUpdate {
    // 由时间推进触发：遍历所有挂载 behavior 的实体并调用 on_time
    pub fn update_by_time(
        register: &LosFuncRegister,
        world: &mut LosWorld,
        elapsed_minutes: f64,
    ) -> Vec<LosEvent> {
        let mut events = Vec::new();
        for (entity, behavior) in register.iter() {
            events.extend(behavior.on_time(world, entity, elapsed_minutes));
        }
        events
    }

    // 动作交互统一调度入口：将 actor 对 target 执行的 way 分发给 target 挂载的所有 behavior
    pub fn apply(
        register: &LosFuncRegister,
        world: &mut LosWorld,
        actor: LosEntity,
        target: LosEntity,
        way: ApplyWays,
    ) -> Result<Vec<LosEvent>, &'static str> {
        let behaviors = register.get(target);
        if behaviors.is_empty() {
            return Err("该物品没有任何响应行为");
        }

        let mut handled = false;
        let mut events = Vec::new();

        for behavior in behaviors {
            if let Some(mut evs) = behavior.on_apply(world, actor, target, way) {
                handled = true;
                events.append(&mut evs);
            }
        }

        if handled {
            Ok(events)
        } else {
            Err("该物品无法响应此使用方式")
        }
    }
}
