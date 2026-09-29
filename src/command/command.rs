use crate::{command::executor::status::ExeStatusMode, component::apply_able::ApplyWays};

#[derive(Debug, Clone, PartialEq)]
pub enum UseTarget {
    ByName(String),
    ById(i64),
}

#[derive(Debug, Clone, PartialEq)]
pub enum LosCommand {
    EXIT, // 退出游戏 quit
    HELP, //
    STOP, // 游戏暂停
    SAVE, // 游戏保存

    STATUS {
        mode: ExeStatusMode,
    }, // 状态指令
    TIME, // 时间指令
    MAP,  // 打印地图
    GO {
        des: String,
    }, // 前往指令

    USE {
        target: UseTarget,
        way: Option<ApplyWays>,
    },

    UNKNOWN(String),
}
