use std::collections::HashMap;

use crate::core::ecs::{LosEntity};


// 存储 全局的 Entity
// 存储其中一个 T 的所有的 组件
// 比如 存健康里面的所有（玩家，怪兽等等）
pub struct LosStorage<T>
{
    _l_data: HashMap<LosEntity,T>
} 



// 读取 存储 
impl<T> LosStorage<T>{
    // new
    pub fn new() -> Self
    {
        Self{
            _l_data: HashMap::new()
        }
    }



    // 插入输入
    pub fn insert(&mut self,entity:LosEntity,value: T)
    {
        self._l_data.insert(entity, value);
    }


    
    // 获得元素
    pub fn get(&self,entity:LosEntity) -> Option<&T>
    {
        self._l_data.get(&entity)
    }



    // 获得可以改变的 元素
    pub fn get_mut(&mut self,entity:LosEntity) -> Option<&mut T>
    {
        self._l_data.get_mut(&entity)
    }
    
}
