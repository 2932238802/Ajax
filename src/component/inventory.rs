use std::collections::HashMap;

use crate::{
    component::{carriable::LosCarriable, item_id::LosItemId},
    ecs::{LosEntity, LosWorld},
};

#[derive(Debug, Clone, PartialEq)]
pub struct LosInventory {
    _l_capacity: f64,
    _l_size: f64,
    pub l_slots: HashMap<LosItemId, Vec<LosEntity>>,
}

impl LosInventory {
    pub fn new(capacity: f64) -> Self {
        Self {
            _l_capacity: capacity,
            _l_size: 0.0,
            l_slots: HashMap::new(),
        }
    }

    pub fn try_push(&mut self, world: &LosWorld, entity: LosEntity) -> Result<usize, &'static str> {
        let item_id = world
            .get_component::<LosItemId>(entity)
            .ok_or("该物品缺少身份组件")?;
        let carriable = world
            .get_component::<LosCarriable>(entity)
            .ok_or("该物品不可携带")?;

        if self._l_size + carriable.l_size > self._l_capacity {
            return Err("背包容量不足");
        }

        self._l_size += carriable.l_size;
        let group = self.l_slots.entry(*item_id).or_default();
        group.push(entity);
        Ok(group.len())
    }

    pub fn try_take(
        &mut self,
        world: &LosWorld,
        kind: LosItemId,
        count: usize,
    ) -> Result<Vec<LosEntity>, &'static str> {
        let mut group = self.l_slots.remove(&kind).ok_or("没有对应的物品")?;
        if group.len() < count {
            self.l_slots.insert(kind, group);
            return Err("数量不足");
        }

        let split_at = group.len() - count;
        let removed = group.split_off(split_at);
        for entity in &removed {
            if let Some(carriable) = world.get_component::<LosCarriable>(*entity) {
                self._l_size = (self._l_size - carriable.l_size).max(0.0);
            }
        }

        if !group.is_empty() {
            self.l_slots.insert(kind, group);
        }
        Ok(removed)
    }

    pub fn show_items(&self, world: &LosWorld) {
        for entities in self.l_slots.values() {
            for entity in entities {
                if let Some(item) = world.get_component::<LosItemId>(*entity) {
                    if let Some(carriable) = world.get_component::<LosCarriable>(*entity) {
                        println!(" ->{}/{} : {}", item.0.get_name(), carriable.l_size, item.0.get_describe());
                    } else {
                        println!(" ->{} : {}", item.0.get_name(), item.0.get_describe());
                    }
                }
            }
        }
    }

    pub fn count(&self, kind: &LosItemId) -> usize {
        self.l_slots.get(kind).map_or(0, Vec::len)
    }

    pub fn is_empty(&self) -> bool {
        self.l_slots.is_empty()
    }

    pub fn capacity(&self) -> f64 {
        self._l_capacity
    }

    pub fn used_size(&self) -> f64 {
        self._l_size
    }
}
