#[derive(Debug, Clone, PartialEq)]
pub enum LosCommand {
    EXIT, // 退出游戏 quit
    HELP, //
    STOP, // 游戏暂停
    SAVE, // 游戏保存

    STATUS, // 状态指令
    MOVE,

    MAP, // 打印地图

    UNKNOWN(String),
}
