use rand::RngExt;

use crate::{
    component::{carriable::LosCarriable, eatable::LosEatable, item_id::LosItemId},
    ecs::{LosEntity, LosWorld},
    entity::kind::LosItem,
};

pub struct Apple;
impl Apple {
    pub fn spawn(world: &mut LosWorld) -> LosEntity {
        let entity = world.spawn();
        world.add_component::<LosItemId>(entity, LosItemId(LosItem::Apple));
        let mut rng = rand::rng();
        world.add_component(
            entity,
            LosCarriable {
                l_size: rng.random_range(1.0..2.0),
            },
        );
        world.add_component(
            entity,
            LosEatable {
                l_health: rng.random_range(0.0..1.0),
                l_hunger: rng.random_range(15.0..25.0),
                l_mental: rng.random_range(0.0..0.1),
            },
        );
        entity
    }
}
