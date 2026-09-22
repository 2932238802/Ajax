use std::rc::Rc;

use crate::core::{event::LosEvent, world::{LosEntity, LosWorld}};

// 游戏状态
pub enum GameState {
    MainMenu, // 主菜单
    Playing,  // 开始游戏
    Settings, // 设置
    Loading,  // 加载状态
    Exit,     // 退出状态
}

// 播放action 的动画
pub const ACTION_ANIMATION: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

// 时间 更替函数
pub type ByTimeFn = Rc<dyn Fn(&mut LosWorld, LosEntity,f64) -> Vec<LosEvent>>;

// 状态更新函数的种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UpdateFuncType
{
    ByTime, // 根据时间
}
