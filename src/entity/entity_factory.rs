use crate::{
    component::item_id::LosItemId,
    ecs::{LosEntity, LosWorld},
    entity::{
        apple, berry,
        item::LosItem::{self},
    },
    system::LosFuncRegister,
};

pub struct EntityFactory;
impl EntityFactory {
    pub fn spawn(world: &mut LosWorld, register: &mut LosFuncRegister, kind: LosItem) -> LosEntity {
        let entity: LosEntity;
        match kind {
            LosItem::Apple => {
                entity = apple::Apple::spawn(world, register);
            }
            LosItem::Berry => {
                entity = berry::Berry::spawn(world, register);
            }
        }
        world.add_component::<LosItemId>(entity, LosItemId(kind));
        entity
    }
}
