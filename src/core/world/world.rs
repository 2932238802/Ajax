use std::{
    any::{Any, TypeId},
    collections::HashMap,
    hash::Hash,
};

use crate::{
    constants::constant_number,
    core::{
        event::LosEvent,
        map::LosMap,
        world::{storage::LosErasedStorage, LosEntity, LosStorage},
    },
};

pub struct LosWorld {
    pub l_map: LosMap,
    _l_next_entity_id: i32,
    _l_storages: HashMap<TypeId, Box<dyn LosErasedStorage>>,
}
impl LosWorld {
    pub fn new() -> Self {
        Self {
            l_map: LosMap::new(
                constant_number::DEFAULT_WIDTH,
                constant_number::DEFAULT_HEIGHT,
            ),
            _l_next_entity_id: 0,
            _l_storages: HashMap::new(), // _l_health_storage: LosStorage<LosHealth>::new(),
        }
    }

    // 返回一个 entity
    pub fn spawn(&mut self) -> LosEntity {
        let entity = LosEntity {
            l_id: self._l_next_entity_id,
        };
        self._l_next_entity_id += 1;
        entity
    }

    // 对一个 entity 增加一个 component
    pub fn add_component<T: 'static>(&mut self, entity: LosEntity, component: T) {
        let type_id = TypeId::of::<T>();
        // 这里的 type_id 就是一个类型
        // or_insert_with 就是如果没有对应的这个类型
        // 就创建一个新的容器
        // entry 返回一个 enum or_insert_with 是返回一个可变值的引用
        let storage = self
            ._l_storages
            .entry(type_id)
            .or_insert_with(|| Box::new(LosStorage::<T>::new()));
        let storage = storage
            .as_any_mut()
            .downcast_mut::<LosStorage<T>>()
            .expect("类型不匹配");
        storage.insert(entity, component);
    }

    // 引用
    pub fn get_component<T: 'static>(&self, entity: LosEntity) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        self._l_storages
            .get(&type_id)?
            .as_any()
            .downcast_ref::<LosStorage<T>>()
            .expect("类型不匹配")
            .get(entity)
    }

    // 获得可以 改变的 component
    // 通过一个实体
    // 一个 类型 映射 类型存储器
    // 类型存储器 里面存的是 entity 对应一个该类型的实际的值
    // TypeId::of::<T>() 获取类型 ID
    // 从 _l_storages 中找到对应类型的存储
    // 如果不存在，创建新的 LosStorage<T>
    // 向下转型为 LosStorage<T>
    // 调用 storage.insert(entity, component) 插入数据
    pub fn get_component_mut<T: 'static>(&mut self, entity: LosEntity) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        self._l_storages
            .get_mut(&type_id)?
            .as_any_mut()
            .downcast_mut::<LosStorage<T>>()
            .expect("类型不匹配")
            .get_mut(entity)
    }

    // 判断一个 entity 的 类型有没有
    pub fn has_component<T: 'static>(&self, entity: LosEntity) -> bool {
        self.get_component::<T>(entity).is_some()
    }

    // 删除 其中 一个 entity 的 属性
    pub fn remove_component<T: 'static>(&mut self, entity: LosEntity) {
        let type_id = TypeId::of::<T>();
        if let Some(storage) = self._l_storages.get_mut(&type_id) {
            if let Some(storage) = storage.as_any_mut().downcast_mut::<LosStorage<T>>() {
                storage.remove_entity(entity);
            }
        }
    }

    // 删除掉 其中一个 entity
    pub fn despawn(&mut self, entity: LosEntity) {
        for storage in self._l_storages.values_mut() {
            storage.remove_entity(entity);
        }
    }
}
