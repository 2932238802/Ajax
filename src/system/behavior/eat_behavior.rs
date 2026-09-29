use rand::{rng, RngExt};

use crate::{
    component::{
        apply_able::ApplyWays, eatable::LosEatable, health::LosAlterHealthOnce,
        hungry::LosAlterHungryOnce, mental::LosAlterMentalOnce, LosHealth, LosHungry, LosMental,
    },
    ecs::{LosEntity, LosWorld},
    game::LosEvent,
    system::behavior::behavior::LosBehavior,
};

pub struct EatBehavior;

impl LosBehavior for EatBehavior {
    fn on_apply(
        &self,
        world: &mut LosWorld,
        actor: LosEntity,
        target: LosEntity,
        way: ApplyWays,
    ) -> Option<Vec<LosEvent>> {
        if way != ApplyWays::EAT || !world.has_component::<LosEatable>(target) {
            return None;
        }

        // 1. 读取食物对各项状态的影响区间（复制数值，释放 world 引用）
        let hunger_range = world
            .get_component::<LosAlterHungryOnce>(target)
            .map(|effect| (effect.l_min, effect.l_max));

        let health_range = world
            .get_component::<LosAlterHealthOnce>(target)
            .map(|effect| (effect.l_min, effect.l_max));

        let mental_range = world
            .get_component::<LosAlterMentalOnce>(target)
            .map(|effect| (effect.l_min, effect.l_max));

        let mut random = rng();
        let hunger_delta = hunger_range.map(|(min, max)| random.random_range(min..max));
        let health_delta = health_range.map(|(min, max)| random.random_range(min..max));
        let mental_delta = mental_range.map(|(min, max)| random.random_range(min..max));

        // 2. 更新使用者（actor）的各组件数值
        if let Some(delta) = hunger_delta {
            if let Some(hungry) = world.get_component_mut::<LosHungry>(actor) {
                hungry.l_current = (hungry.l_current + delta).min(hungry.l_max);
            }
        }
        if let Some(delta) = health_delta {
            if let Some(health) = world.get_component_mut::<LosHealth>(actor) {
                health.l_current = (health.l_current + delta).min(health.l_max);
            }
        }
        if let Some(delta) = mental_delta {
            if let Some(mental) = world.get_component_mut::<LosMental>(actor) {
                mental.l_current = (mental.l_current + delta).min(mental.l_max);
            }
        }

        Some(Vec::new())
    }
}
