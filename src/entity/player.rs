use std::rc::Rc;

use crate::{
    component::{inventory::LosInventory, LosHealth, LosHungry, LosMental, LosPosition},
    constants::constant_number::PLAYER_INVENTORY_CAPACITY,
    ecs::{LosEntity, LosWorld},
    game::{event::{DeathEvent, LosEvent, OnTheVergeEvent}, save::SavePlayer},
    system::{LosBehavior, LosFuncRegister},
};

// 提供一个静态函数
pub struct LosPlayer {}

// 玩家的生成
impl LosPlayer {
    pub fn spawn(world: &mut LosWorld, register: &mut LosFuncRegister) -> LosEntity {
        let entity = world.spawn();
        world.add_component(entity, LosHealth::default());
        world.add_component(entity, LosHungry::default());
        world.add_component(entity, LosMental::default());
        world.add_component(
            entity,
            // 默认出生在家里
            LosPosition {
                l_position: crate::world::LosTerrain::Home,
            },
        );
        world.add_component(entity, LosInventory::new(PLAYER_INVENTORY_CAPACITY));
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

        let mut health = LosHealth::default();
        health.l_current = data.l_state.l_cur_health;
        health.l_max = data.l_state.l_health_max;
        health.l_hunger_damage_rate = data.l_state.l_hunger_damage_rate;
        health.l_mental_damage_rate = data.l_state.l_mental_damage_rate;
        world.add_component(entity, health);

        let mut hungry = LosHungry::default();
        hungry.l_current = data.l_state.l_cur_hungry;
        hungry.l_max = data.l_state.l_hungry_max;
        hungry.l_decay_rate = data.l_state.l_hungry_decay_rate;
        world.add_component(entity, hungry);

        let mut mental = LosMental::default();
        mental.l_current = data.l_state.l_cur_mental;
        mental.l_max = data.l_state.l_mental_max;
        mental.l_decay_rate = data.l_state.l_mental_decay_rate;
        world.add_component(entity, mental);

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
            .map(|h| h.l_current < h.l_starving_threshold)
            .unwrap_or(false);
        let broken = world
            .get_component::<LosMental>(entity)
            .map(|m| m.l_current < m.l_broken_threshold)
            .unwrap_or(false);
        if let Some(health) = world.get_component_mut::<LosHealth>(entity) {
            if starving {
                health.l_current -= health.l_hunger_damage_rate * elapsed;
            }
            if broken {
                health.l_current -= health.l_mental_damage_rate * elapsed;
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
            hungry.l_current = (hungry.l_current - hungry.l_decay_rate * elapsed).max(0.0);
            if hungry.l_current <= hungry.l_starving_threshold {
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
            mental.l_current = (mental.l_current - mental.l_decay_rate * elapsed).max(0.0);
            if mental.l_current <= mental.l_verge_threshold {
                events.push(LosEvent::OnTheVerge {
                    entity,
                    cause: OnTheVergeEvent::MentalOnTheVerge,
                });
            }
        }
        events
    }
}
