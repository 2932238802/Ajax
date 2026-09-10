use std::{any::{Any, TypeId}, collections::HashMap};

use crate::{component::{self, LosHealth, LosPosition}, core::ecs::{LosEntity, LosStorage}};



pub struct LosWorld
{
    _l_next_entity_id: u32,
    _l_health_storage: LosStorage<LosHealth>,
    _l_position_storage: LosStorage<LosPosition>,
    // 类型 -> 指针的映射
    // 这里的指针指向的是 LosStorage<LosPosition> LosStorage<LosHealth> ...
    _l_storages: HashMap<TypeId,Box<dyn Any>> 
}



// 
impl LosWorld {
    pub fn new() -> Self
    {
        Self{
            _l_next_entity_id:0,
            _l_health_storage: LosStorage::<LosHealth>::new(),
            // _l_health_storage: LosStorage<LosHealth>::new(),
            // 这两种写法都是 合法的 
            // 
            _l_position_storage: LosStorage::<LosPosition>::new(),
            _l_storages: HashMap::new()
        }
    }



    // 返回一个 LosEntity
    pub fn spawn(&mut self) -> LosEntity
    {
        let entity = LosEntity{
            l_id: self._l_next_entity_id
        };
        self._l_next_entity_id += 1;
        entity
    }



    // 增加一个 的组件
    pub fn add_component<T: 'static>(&mut self,entity: LosEntity,component:T)
    {
        let type_id = TypeId::of::<T>();

        let storage = self._l_storages.entry(
            type_id
        ).or_insert_with(|| Box::new(LosStorage::<T>::new()));
        

    }
}