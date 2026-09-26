use std::{collections::HashMap, rc::Rc};

use crate::core::{behavior::LosBehavior, world::LosEntity};

// 行为注册表
// 为每个 entity 挂载若干行为（LosBehavior）
// 时间推进 走 iter()，使用触发 走 get()
pub struct LosFuncRegister {
    _l_behaviors: HashMap<LosEntity, Vec<Rc<dyn LosBehavior>>>,
}

impl LosFuncRegister {
    pub fn new() -> Self {
        Self {
            _l_behaviors: HashMap::new(),
        }
    }

    // 给 entity 挂载一个行为
    pub fn attach(&mut self, entity: LosEntity, behavior: Rc<dyn LosBehavior>) {
        self._l_behaviors.entry(entity).or_default().push(behavior);
    }

    // 移除 entity 的所有行为（实体销毁时调用）
    pub fn remove_entity(&mut self, entity: LosEntity) {
        self._l_behaviors.remove(&entity);
    }

    // 遍历所有 entity 的行为
    // 时间推进用
    pub fn iter(&self) -> impl Iterator<Item = (LosEntity, &Rc<dyn LosBehavior>)> + '_ {
        self._l_behaviors
            .iter()
            .flat_map(|(entity, behaviors)| behaviors.iter().map(move |b| (*entity, b)))
    }

    // 取某个 entity 的行为（使用触发用）
    pub fn get(&self, entity: LosEntity) -> Vec<&Rc<dyn LosBehavior>> {
        self._l_behaviors
            .get(&entity)
            .map(|behaviors| behaviors.iter().collect())
            .unwrap_or_default()
    }
}
