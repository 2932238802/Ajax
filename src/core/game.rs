use std::io;

use crate::{core::ecs::{LosEntity, LosWorld}, entity::player::LosPlayer};

// 主要游戏界面
struct LosGame {
    l_world: LosWorld,
    l_player: LosEntity,
    l_running: bool,
}

// 输入输出循环
impl LosGame 
{
    // 生成游戏
    pub fn spawn() -> Self
    {
        let mut world = LosWorld::new();
        let player = LosPlayer::spawn(&mut world);
        Self
        {
            l_world:world,
            l_player:player,
            l_running:true,
        }
    }

    // 开始运行
    pub fn run()
    {
        let mut input =  String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
    }
}
