use crate::{component::{LosHealth, LosPosition}, core::ecs::{LosEntity, LosStorage}};



pub struct LosWorld
{
    _l_next_entity_id: u32,
    _l_health_storage: LosStorage<LosHealth>,
    _l_position_storage: LosStorage<LosPosition>
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
            _l_position_storage: LosStorage::<LosPosition>::new()
        }
    }



    // 返回一个 entity
    pub fn spawn(&mut self) -> LosEntity
    {
        let entity = LosEntity{
            l_id: self._l_next_entity_id
        };
        self._l_next_entity_id += 1;
        entity
    }



    // 增加一个 health的组件
    pub fn add_entity<A,B>
    {

    }
}