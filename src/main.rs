pub mod command;
pub mod common;
pub mod component;
pub mod constants;
pub mod ecs;
pub mod entity;
pub mod game;
pub mod system;
pub mod world;
use crate::game::LosGame;

fn main() {
    let mut game = LosGame::new();
    game.run();
}
