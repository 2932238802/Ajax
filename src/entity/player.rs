use std::rc::Rc;

use crate::{
    component::{
        carriable::LosCarriable, inventory::LosInventory, item_id::LosItemId, LosHealth, LosHungry,
        LosMental, LosPosition,
    },
    constants::constant_number::PLAYER_INVENTORY_CAPACITY,
    ecs::{LosEntity, LosWorld},
    entity::{entity_factory::EntityFactory, item::LosItem},
    game::save::SavePlayer,
    system::{
        behavior::{
            health_behavior::HealthBehavior, hungry_behavior::HungerBehavior,
            mental_behavior::MentalBehavior,
        },
        LosFuncRegister,
    },
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

        let mut inventory = LosInventory::new(PLAYER_INVENTORY_CAPACITY);
        let apple_id = LosItemId(LosItem::Apple);
        let apples = EntityFactory::spawn_many(world, register, LosItem::Apple, 3)
            .expect("创建初始苹果失败");

        for apple in apples {
            let size = world
                .get_component::<LosCarriable>(apple)
                .expect("苹果缺少 LosCarriable 组件")
                .l_size;

            inventory
                .try_push(&apple_id, size, apple)
                .expect("初始苹果放入玩家携带空间失败");
        }

        world.add_component(entity, inventory);
        Self::register_func(register, &entity);
        entity
    }

    // 给玩家挂载行为
    pub fn register_func(register: &mut LosFuncRegister, entity: &LosEntity) {
        register.attach(*entity, Rc::new(HealthBehavior));
        register.attach(*entity, Rc::new(HungerBehavior));
        register.attach(*entity, Rc::new(MentalBehavior));
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
        world.add_component(entity, LosInventory::new(PLAYER_INVENTORY_CAPACITY));
        Self::register_func(register, &entity);
        entity
    }
}
