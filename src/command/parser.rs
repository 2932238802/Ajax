use std::char::ToLowercase;

use serde::de;

use crate::{
    command::{commands::commands::LosCommand, executor::status::ExeStatusMode},
    core::map::LosMap,
};

// 分隔器
// input_command 是输入的指令
pub fn parser(input_command: &str) -> LosCommand {
    let input_command = input_command.trim();
    if input_command.is_empty() {
        return LosCommand::UNKNOWN(String::new());
    }
    let mut parts = input_command.split_whitespace();
    let number = parts.clone().count();
    let command = match parts.next() {
        Some(command) => command.to_lowercase(),
        None => return LosCommand::UNKNOWN(String::new()),
    };

    // #[derive(Debug, Clone, PartialEq)]
    // pub enum LosCommand {
    //     EXIT, // 退出游戏 quit
    //     HELP, //
    //     STOP, // 游戏暂停
    //     SAVE, // 游戏保存

    //     STATUS, // 状态指令
    //     TIME,               // 时间指令
    //     MAP,                // 打印地图
    //     GO { des: String }, // 前往指令

    //     UNKNOWN(String),
    // }

    match command.as_str() {
        "quit" | "exit" | "q" => LosCommand::EXIT,

        "help" | "h" => LosCommand::HELP,

        "stop" | "p" => LosCommand::STOP,

        "save" => LosCommand::SAVE,

        "status" | "s" => {
            if number > 2 {
                return error_use("status");
            }
            if number == 1 {
                return LosCommand::STATUS {
                    mode: ExeStatusMode::ALL,
                };
            } else {
                match parts.next() {
                    Some(dest) => {
                        if dest == "u" {
                            return LosCommand::STATUS {
                                mode: ExeStatusMode::HUNGRY,
                            };
                        } else if dest == "h" {
                            return LosCommand::STATUS {
                                mode: ExeStatusMode::HEALTH,
                            };
                        } else if dest == "m" {
                            return LosCommand::STATUS {
                                mode: ExeStatusMode::MENTAL,
                            };
                        } else if dest.parse::<i64>().is_ok() {
                            return LosCommand::STATUS {
                                mode: ExeStatusMode::BYENTITY,
                            };
                        } else {
                            return LosCommand::UNKNOWN("不合法的status第二个参数".to_string());
                        }
                    }
                    None => LosCommand::UNKNOWN("status's args".to_string()),
                }
            }
        }

        "time" | "t" => LosCommand::TIME,

        "map" | "m" => LosCommand::MAP,

        "go" | "g" => {
            if number != 2 {
                return error_use("go");
            }
            match parts.next() {
                Some(dest) => {
                    if LosMap::isvalid(dest) {
                        LosCommand::GO {
                            // 返回 go
                            des: dest.to_string(),
                        }
                    } else {
                        return error_use("go's args");
                    }
                }
                None => LosCommand::UNKNOWN("缺少目标地点".to_string()),
            }
        }
        _ => LosCommand::UNKNOWN(input_command.to_string()),
    }
}

// 错误 使用的时候 提示
fn error_use(word: &str) -> LosCommand {
    println!(
        "!!! {} 使用时发生错误, 可以使用 help -s {} 的方式查看使用方法以及对应的参数",
        word, word
    );
    LosCommand::UNKNOWN("".to_string())
}
