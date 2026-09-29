use crate::{
    component::{
        apply_able::LosApplyAble, carriable::LosCarriable, eatable::LosEatable,
        health::LosAlterHealthOnce, hungry::LosAlterHungryOnce, item_id::LosItemId,
        mental::LosAlterMentalOnce,
    },
    ecs::{LosEntity, LosWorld},
    entity::item::LosItem,
    system::{behavior::eat_behavior::EatBehavior, LosFuncRegister},
};
use std::rc::Rc;

pub struct LosApple;

// LosCarriable
impl LosApple {
    pub fn spawn(world: &mut LosWorld, register: &mut LosFuncRegister) -> LosEntity {
        let entity = world.spawn();
        world.add_component::<LosItemId>(entity, LosItemId(LosItem::Apple));
        world.add_component(entity, LosCarriable { l_size: 0.5 });
        world.add_component(entity, LosEatable);
        world.add_component(
            entity,
            LosAlterHungryOnce {
                l_min: 15.0,
                l_max: 25.0,
            },
        );
        world.add_component(
            entity,
            LosAlterHealthOnce {
                l_min: 0.0,
                l_max: 1.0,
            },
        );
        world.add_component(
            entity,
            LosAlterMentalOnce {
                l_min: 0.0,
                l_max: 0.1,
            },
        );
        world.add_component(entity, LosApplyAble {});
        register.attach(entity, Rc::new(EatBehavior));
        entity
    }
}
