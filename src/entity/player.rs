use std::rc::Rc;

use crate::{
    component::{LosHealth, LosHungry, LosMental, LosPosition},
    constants::constant_number::{
        HUNGRY_AFFECT_HEALTH_MULTY, HUNGRY_MUTLY, MENTAL_AFFECT_HEALTH_MULTY, MENTAL_MUTLY,
    },
    core::{
        behavior::LosBehavior,
        data::SavePlayer,
        event::{DeathEvent, LosEvent, OnTheVergeEvent},
        register::LosFuncRegister,
        world::{LosEntity, LosWorld},
    },
};

// 提供一个静态函数
pub struct LosPlayer {}

// 玩家的生成
impl LosPlayer {
    pub fn spawn(world: &mut LosWorld, register: &mut LosFuncRegister) -> LosEntity {
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
                l_position: crate::core::terrain::LosTerrain::Home,
            },
        );
        Self::register_func(register, &entity);
        entity
    }

    // 给玩家挂载行为
    pub fn register_func(register: &mut LosFuncRegister, entity: &LosEntity) {
        register.attach(*entity, Rc::new(HealthDecay));
        register.attach(*entity, Rc::new(HungerDecay));
        register.attach(*entity, Rc::new(MentalDecay));
    }

    // 从 保存的数据里 来
    pub fn from_save_data(
        data: &SavePlayer,
        world: &mut LosWorld,
        register: &mut LosFuncRegister,
    ) -> LosEntity {
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
                l_current: data.l_state.l_cur_hungry,
                l_max: data.l_state.l_hungry_max,
            },
        );
        world.add_component(
            entity,
            LosMental {
                l_current: data.l_state.l_cur_mental,
                l_max: data.l_state.l_mental_max,
            },
        );
        world.add_component(
            entity,
            // 默认出生在家里
            LosPosition {
                l_position: data.l_position.l_position,
            },
        );
        Self::register_func(register, &entity);
        entity
    }
}

struct HealthDecay;

impl LosBehavior for HealthDecay {
    fn on_time(&self, world: &mut LosWorld, entity: LosEntity, elapsed: f64) -> Vec<LosEvent> {
        let mut events = Vec::new();
        let starving = world
            .get_component::<LosHungry>(entity)
            .map(|h| h.l_current < 10.0)
            .unwrap_or(false);
        let broken = world
            .get_component::<LosMental>(entity)
            .map(|m| m.l_current < 15.0)
            .unwrap_or(false);
        if let Some(health) = world.get_component_mut::<LosHealth>(entity) {
            if starving {
                health.l_current -= HUNGRY_AFFECT_HEALTH_MULTY * elapsed;
            }
            if broken {
                health.l_current -= MENTAL_AFFECT_HEALTH_MULTY * elapsed;
            }
            if health.l_current <= 0.0 {
                events.push(LosEvent::Death {
                    entity,
                    cause: DeathEvent::Starvation,
                });
            }
        }
        events
    }
}

struct HungerDecay;

impl LosBehavior for HungerDecay {
    fn on_time(&self, world: &mut LosWorld, entity: LosEntity, elapsed: f64) -> Vec<LosEvent> {
        let mut events = Vec::new();
        if let Some(hungry) = world.get_component_mut::<LosHungry>(entity) {
            hungry.l_current = (hungry.l_current - HUNGRY_MUTLY * elapsed).max(0.0);
            if hungry.l_current <= 10.0 {
                events.push(LosEvent::OnTheVerge {
                    entity,
                    cause: OnTheVergeEvent::HungryOnTheVerge,
                });
            }
        }
        events
    }
}

// 精神：随时间下降
struct MentalDecay;

impl LosBehavior for MentalDecay {
    fn on_time(&self, world: &mut LosWorld, entity: LosEntity, elapsed: f64) -> Vec<LosEvent> {
        let mut events = Vec::new();
        if let Some(mental) = world.get_component_mut::<LosMental>(entity) {
            mental.l_current = (mental.l_current - MENTAL_MUTLY * elapsed).max(0.0);
            if mental.l_current <= 40.0 {
                events.push(LosEvent::OnTheVerge {
                    entity,
                    cause: OnTheVergeEvent::MentalOnTheVerge,
                });
            }
        }
        events
    }
}
