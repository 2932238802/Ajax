use crate::{
    command::{
        command::LosCommand,
        executor::{
            apply::apply, bag::bag, exit::exit, go::go, help::help, save::save, show_map::show_map,
            status::status, time::time,
        },
    },
    game::LosGame,
};

// 指令路由器
pub fn router(command: LosCommand, game: &mut LosGame) {
    match command {
        LosCommand::EXIT => {
            exit(game);
        }
        LosCommand::HELP => {
            help();
        }
        LosCommand::STOP => {}
        LosCommand::SAVE => {
            save(game);
        }

        LosCommand::BAG => {
            bag(game);
        }
        LosCommand::GO { des } => {
            go(game, &des);
        }
        LosCommand::MAP => {
            show_map(game);
        }
        LosCommand::STATUS { mode } => {
            status(game, mode);
        }
        LosCommand::TIME => {
            time(game);
        }

        LosCommand::USE { target, way } => {
            apply(game, target, way);
        }

        LosCommand::UNKNOWN(_) => {}
    }
}
