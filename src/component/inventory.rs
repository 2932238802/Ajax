use std::collections::HashMap;

use crate::{component::item_id::LosItemId, ecs::LosEntity};
#[derive(Debug, Clone, PartialEq)]
pub struct LosInventory {
    _l_capacity: f64,
    _l_size: f64,
    _l_slots: HashMap<LosItemId, Vec<LosEntity>>,
}

impl LosInventory {
    pub fn new(capacity: f64) -> Self {
        Self {
            _l_capacity: capacity,
            _l_size: 0.0,
            _l_slots: HashMap::new(),
        }
    }

    pub fn try_push(
        &mut self,
        id: &LosItemId,
        size: f64,
        entity: LosEntity,
    ) -> Result<usize, &'static str> {
        if self._l_size + size > self._l_capacity {
            return Err("背包容量不足");
        }
        self._l_size += size;
        let group = self._l_slots.entry(*id).or_default();
        group.push(entity);
        Ok(group.len())
    }

    pub fn try_take(
        &mut self,
        kind: &LosItemId,
        count: usize,
    ) -> Result<Vec<LosEntity>, &'static str> {
        if count == 0 {
            return Err("取出数量必须大于 0");
        }
        let mut group = self._l_slots.remove(kind).ok_or("没有对应的物品种类")?;
        if group.len() < count {
            self._l_slots.insert(*kind, group);
            return Err("物品数量不足");
        }
        let split_at = group.len() - count;
        let removed = group.split_off(split_at);
        if !group.is_empty() {
            self._l_slots.insert(*kind, group);
        }
        Ok(removed)
    }

    // 取出一个
    pub fn try_take_one(&mut self, kind: &LosItemId) -> Result<LosEntity, &'static str> {
        self.try_take(kind, 1)?.pop().ok_or("取出失败：内部错误")
    }

    // 展示 背包的物品
    // name 就是 持有 inventory 的 entity
    pub fn show_items(&self, name: &str) {
        println!(" ==== {} 内容为 ==== ", name);
        for (left, right) in self._l_slots.iter() {
            println!(" {} | {}", left.0.get_name(), right.len());
        }
        println!(" ==== {} 展示内容结束 ==== ", name);
    }

    pub fn has(&self, entity: LosEntity) -> Option<LosItemId> {
        for (item_id, right) in &self._l_slots {
            for entity_ in right {
                if entity_.clone() == entity {
                    return Some(item_id.clone());
                }
            }
        }
        None
    }

    // 迭代
    pub fn iter_items(&self) -> impl Iterator<Item = (&LosItemId, &[LosEntity])> + '_ {
        self._l_slots
            .iter()
            .map(|(kind, entities)| (kind, entities.as_slice()))
    }

    // 查看一个 不拿走
    pub fn peek(&self, kind: &LosItemId) -> Option<LosEntity> {
        self._l_slots
            .get(kind)
            .and_then(|group| group.last().copied())
    }

    pub fn release_size(&mut self, size: f64) {
        self._l_size = (self._l_size - size).max(0.0);
    }

    pub fn count(&self, kind: &LosItemId) -> usize {
        self._l_slots.get(kind).map_or(0, Vec::len)
    }

    pub fn is_empty(&self) -> bool {
        self._l_slots.is_empty()
    }

    pub fn capacity(&self) -> f64 {
        self._l_capacity
    }

    pub fn used_size(&self) -> f64 {
        self._l_size
    }
}
