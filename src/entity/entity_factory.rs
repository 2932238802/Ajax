use crate::{
    ecs::{LosEntity, LosWorld},
    entity::{apple, berry, item::LosItem},
    system::LosFuncRegister,
};

pub struct EntityFactory;

impl EntityFactory {
    /// 创建一个具体物品实体。
    pub fn spawn(
        world: &mut LosWorld,
        register: &mut LosFuncRegister,
        kind: LosItem,
    ) -> Result<LosEntity, &'static str> {
        match kind {
            LosItem::Apple => Ok(apple::LosApple::spawn(world, register)),
            LosItem::Berry => Ok(berry::LosBerry::spawn(world, register)),
            LosItem::None => Err("不能创建空物品"),
        }
    }

    /// 批量创建同一种物品。
    pub fn spawn_many(
        world: &mut LosWorld,
        register: &mut LosFuncRegister,
        kind: LosItem,
        count: usize,
    ) -> Result<Vec<LosEntity>, &'static str> {
        (0..count)
            .map(|_| Self::spawn(world, register, kind))
            .collect()
    }
}
