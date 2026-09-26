use crate::core::{
    event::LosEvent,
    register::LosFuncRegister,
    world::{LosEntity, LosWorld},
};

pub struct LosUpdate;

impl LosUpdate {
    // 由时间推进触发：遍历所有行为
    // 调用 on_time
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

    // 只触发目标实体的行为
    pub fn update_by_use(
        register: &LosFuncRegister,
        world: &mut LosWorld,
        user: LosEntity,
        target: LosEntity,
    ) -> Vec<LosEvent> {
        let mut events = Vec::new();
        for behavior in register.get(target) {
            events.extend(behavior.on_use(world, user, target));
        }
        events
    }
}
