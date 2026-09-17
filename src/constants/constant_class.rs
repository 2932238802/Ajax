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
