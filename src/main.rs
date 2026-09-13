
pub mod component;
pub mod core;
pub mod system;
pub mod entity;
pub mod command;
pub mod constants;

use crate::core::{
    game::LosGame
};

fn main()
{
    let mut g:core::game::LosGame = LosGame::new();
    g.run();
}

