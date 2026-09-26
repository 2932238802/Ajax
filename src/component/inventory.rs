    use std::{collections::HashMap, hash::Hash};

    use serde::{Deserialize, Serialize};

    use crate::{
        component::{carriable::LosCarriable, item_id::LosItemId},
        constants::constant_number,
        core::world::{LosEntity, LosWorld},
    };

    // 表示一个物品可以存储
    #[derive(Debug, Clone, PartialEq)]
    pub struct LosInventory {
        _l_capacity: f64,
        _l_size: f64,
        pub l_slots: HashMap<LosItemId, Vec<LosEntity>>,
    }

    impl LosInventory {
        pub fn new(capacity: usize) -> Self {
            Self {
                _l_capacity: capacity as f64,
                _l_size: 0.0,
                l_slots: HashMap::new(),
            }
        }

        // 得到 尺寸
        pub fn get_capacity(&self) -> u64 {
            self._l_capacity as u64
        }

        // 尝试放入
        pub fn try_push(&mut self, world: &LosWorld, entity: LosEntity) -> Result<usize, &'static str> {
            // 1. 读身份（用来决定放进哪个 key）和可携带信息（用来算重量）
            let item_id = world
                .get_component::<LosItemId>(entity)
                .ok_or(" !该物品缺少身份组件")?;
            let carriable = world
                .get_component::<LosCarriable>(entity)
                .ok_or(" !该物品不可携带")?;
            if self._l_size + carriable.l_size > self._l_capacity {
                return Err(" !背包容量不足");
            }
            self._l_size += carriable.l_size;
            let group = self.l_slots.entry(*item_id).or_insert_with(Vec::new);
            group.push(entity);
            Ok(group.len())
        }



        // 得到 包 里面的东西
        pub fn show_items(&self, world: &LosWorld) {
            for item in &self.l_slots {
                for entity in item.1 {
                    if let Some(i) = world.get_component::<LosItemId>(*entity) {
                        if let Some(i_carrible) = world.get_component::<LosCarriable>(*entity) {
                            println!(
                                " ->{}/{} : {}",
                                i.0.get_name(),
                                i_carrible.l_size,
                                i.0.get_describe()
                            );
                        } else
                        // 不可 堆叠 打印方式
                        {
                            println!(" ->{} : {}", i.0.get_name(), i.0.get_describe());
                        }
                    }
                }
            }
        }
    }
