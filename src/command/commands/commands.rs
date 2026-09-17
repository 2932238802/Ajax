#[derive(Debug, Clone, PartialEq)]
pub enum LosCommand {
    EXIT, // 退出游戏 quit
    HELP, //
    STOP, // 游戏暂停
    SAVE, // 游戏保存

    STATUS, // 状态指令
    TIME,               // 时间指令
    MAP,                // 打印地图
    GO { des: String }, // 前往指令

    UNKNOWN(String),
}
