pub mod command;
pub mod component;
pub mod constants;
pub mod core;
pub mod entity;
pub mod system;

use crate::core::game::LosGame;

fn main() {
    let mut g: core::game::LosGame = LosGame::new();
    g.run();
}
