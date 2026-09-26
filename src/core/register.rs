use std::{collections::HashMap, hash::Hash};

use crate::{
    constants::constant_class::{
        ByTimeFn,
        UpdateFuncType::{self, ByTime},
    },
    core::world::LosEntity,
};

#[derive(Clone)]
pub enum RegisteredFunc {
    ByTime(ByTimeFn),
}

impl RegisteredFunc {
    // 函数匹配
    pub fn func_type(&self) -> UpdateFuncType {
        match self {
            RegisteredFunc::ByTime(_) => UpdateFuncType::ByTime,
        }
    }
}

pub struct LosFuncRegister {
    // 通过 entity 以及 想要找到更新的函数的类型 找到对应的业务函数
    // 比如进食
    // 比如战斗
    _l_entity_funcs: HashMap<(LosEntity, UpdateFuncType), Vec<RegisteredFunc>>,

    // 通过 更新的类型 拿到 entity 以及 对应的业务函数
    // 比如时间更新 比如 物质生产
    _l_type_funcs: HashMap<UpdateFuncType, Vec<(LosEntity, RegisteredFunc)>>,
}

impl LosFuncRegister {
    pub fn new() -> Self {
        Self {
            _l_entity_funcs: HashMap::new(),
            _l_type_funcs: HashMap::new(),
        }
    }

    // 注册函数
    pub fn register(&mut self, entity: LosEntity, func: RegisteredFunc) {
        let func_type = func.func_type();
        self._l_entity_funcs
            .entry((entity, func_type))
            .or_insert_with(Vec::new)
            .push(func.clone()); // RC 支持Clone
        self._l_type_funcs
            .entry(func_type)
            .or_insert_with(Vec::new)
            .push((entity, func));
    }

    pub fn get_by_func_type(&self, func_type: UpdateFuncType) -> Vec<(LosEntity, &RegisteredFunc)> {
        self._l_type_funcs
            .get(&func_type)
            .map(|funcs| funcs.iter().map(|(entity, func)| (*entity, func)).collect())
            .unwrap_or_default()
    }

    // 进食 战斗
    // 需要拿到 食物的更新函数
    // entity 物品 的 entity
    // func_type 物品的功能
    pub fn get_by_entity_and_type(
        &self,
        entity: LosEntity,
        func_type: UpdateFuncType,
    ) -> Vec<&RegisteredFunc> {
        self._l_entity_funcs
            .get(&(entity, func_type))
            .map(|funcs| funcs.iter().collect())
            .unwrap_or_default()
    }

    // 移除 对应 entity 的所有函数
    pub fn remove_entity(&mut self, entity: LosEntity) {
        self._l_entity_funcs.retain(|(e, _), _| *e != entity); // 保留 不是 entity 的 也就是 删掉 所有的 entity
                                                               // _l_type_funcs: HashMap<UpdateFuncType, Vec<(LosEntity, RegisteredFunc)>>,
        for right in self._l_type_funcs.values_mut() {
            right.retain(|(e, _)| *e != entity);
        }
    }
}
