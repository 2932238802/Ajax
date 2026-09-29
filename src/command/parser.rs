use crate::{
    command::{
        command::{LosCommand, UseTarget},
        executor::status::ExeStatusMode,
    },
    component::apply_able::ApplyWays,
    world::LosMap,
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

        "use" | "apply" | "us" | "u" => {
            let target_str = match parts.next() {
                Some(t) => t,
                None => return error_use("use"),
            };

            let target = if let Ok(id) = target_str.parse::<i64>() {
                UseTarget::ById(id)
            } else {
                UseTarget::ByName(target_str.to_lowercase())
            };

            let way = parts.next().and_then(|w| match w.to_lowercase().as_str() {
                "eat" | "e" => Some(ApplyWays::EAT),
                "give" | "g" => Some(ApplyWays::GIVE),
                _ => None,
            });

            LosCommand::USE { target, way }
        }
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
