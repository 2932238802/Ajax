use crate::{
    command::{
        commands::commands::LosCommand,
        executor::
        {
            exit::exit, 
            help::help, 
            save::save, 
            show_map::show_map,
            go::go
        },
    },
    core::{ecs::LosWorld, game::LosGame},
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

        LosCommand::STATUS => {}
        LosCommand::MOVE => {}
        LosCommand::MAP => {
            show_map(game);
        }

        LosCommand::GO{des} =>
        {
            go(game,&des);
        }

        LosCommand::UNKNOWN(_) => {}
    }
}
