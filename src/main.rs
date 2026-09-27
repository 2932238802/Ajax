pub mod command;
pub mod component;
pub mod constants;
pub mod ecs;
pub mod entity;
pub mod game;
pub mod item;
pub mod system;
pub mod world;

use crate::game::LosGame;

fn main() {
    let mut game = LosGame::new();
    game.run();
}
