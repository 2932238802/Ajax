use std::rc::Rc;

use crate::{
    component::{
        carriable::LosCarriable, eatable::LosEatable, hungry::LosAlterHungryOnce,
        item_id::LosItemId,
    },
    ecs::{LosEntity, LosWorld},
    entity::item::LosItem,
    system::{behavior::eat_behavior::EatBehavior, LosFuncRegister},
};

pub struct LosBerry;
impl LosBerry {
    pub fn spawn(world: &mut LosWorld, register: &mut LosFuncRegister) -> LosEntity {
        let entity = world.spawn();
        world.add_component::<LosItemId>(entity, LosItemId(LosItem::Berry));
        world.add_component(entity, LosCarriable { l_size: 0.2 });
        world.add_component(entity, LosEatable);
        world.add_component(
            entity,
            LosAlterHungryOnce {
                l_min: 5.0,
                l_max: 10.0,
            },
        );
        register.attach(entity, Rc::new(EatBehavior));
        entity
    }
}
