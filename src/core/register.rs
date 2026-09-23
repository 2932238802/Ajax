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
    _l_entity_funcs: HashMap<(LosEntity, UpdateFuncType), Vec<RegisteredFunc>>,
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
}
