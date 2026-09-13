use crate::command::commands::commands::LosCommand;

// 分隔器
// input_command 是输入的指令
pub fn parser(input_command: &str) -> LosCommand {
    let input_command = input_command.trim();
    if input_command.is_empty() {
        return LosCommand::UNKNOWN(String::new());
    }
    let mut parts = input_command.split_whitespace();
    let command = match parts.next() {
        Some(command) => command.to_lowercase(),
        None => return LosCommand::UNKNOWN(String::new()),
    };
    match command.as_str() {
        "help" | "h" => LosCommand::HELP,

        "status" | "s" => LosCommand::STATUS,

        "map" | "m" => LosCommand::MAP,

        "quit" | "exit" | "q" => LosCommand::EXIT,

        _ => LosCommand::UNKNOWN(input_command.to_string()),
    }
}
