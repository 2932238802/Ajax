use crate::{command::commands::commands::LosCommand, core::map::map::LosMap};

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
        "help" | "h" => LosCommand::HELP,

        "status" | "s" => LosCommand::STATUS,

        "map" | "m" => LosCommand::MAP,

        "quit" | "exit" | "q" => LosCommand::EXIT,

        "go" | "g" => {
            if number != 2 {
                return error_use("go");
            }
            match parts.next() {
                Some(dest) => {
                    if LosMap::isvalid(dest) {
                        LosCommand::GO { // 返回 go
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
