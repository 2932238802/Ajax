use std::{
    eprintln, fs,
    io::{self, Write},
    print, println,
};

use crate::{
    command::{parser::parser, router::router},
    component::{LosHealth, LosPosition},
    constants::{constant_class::GameState, constant_str},
    core::{
        ecs::{LosEntity, LosWorld},
        save::data::{SaveData, SaveMap, SavePlayer, SaveState},
    },
    entity::player::LosPlayer,
};

// 主要游戏界面
pub struct LosGame {
    pub l_world: LosWorld,
    pub l_player: LosEntity,
    l_running: bool,
    l_game_state: GameState,
}

// 输入输出循环
impl LosGame {
    // 生成游戏
    pub fn new() -> Self {
        let mut world = LosWorld::new();
        let player = LosPlayer::spawn(&mut world);

        Self {
            l_world: world,
            l_player: player,
            l_running: true,
            l_game_state: GameState::MainMenu,
        }
    }

    // 开始运行
    // 打印基本的信息
    pub fn run(&mut self) {
        println!("{}", constant_str::WELCOME_STR);

        while self.l_running {
            match self.l_game_state {
                GameState::Loading => {
                    self._loading();
                }

                GameState::MainMenu => {
                    self._mainmenu();
                }

                GameState::Playing => {
                    self._playing();
                }

                GameState::Settings => {
                    self._settings();
                }

                GameState::Exit => {
                    self.l_running = false;
                }
            }
        }

        println!("程序已退出");
    }

    // 退出游戏
    pub fn exit(&mut self) {
        self.l_game_state = GameState::Exit;
    }

    // 保存游戏
    // 游戏中 可以主动 调用 进行一个保存
    pub fn save(&self) {
        let position = self
            .l_world
            .get_component::<LosPosition>(self.l_player)
            .expect("玩家缺少位置组件");

        let player_health_state = self
            .l_world
            .get_component::<LosHealth>(self.l_player)
            .expect("玩家缺少健康状态组件");

        let save_state = SaveData {
            l_map: SaveMap {
                l_map: self.l_world.l_map.get_data().to_vec(),
            },

            l_player: SavePlayer {
                l_entity_id: self.l_player.l_id,
                l_position: position.clone(),

                l_state: SaveState {
                    l_cur_health: player_health_state.l_current,
                    l_cur_max_health: player_health_state.l_max,
                },
            },
        };

        let json = match serde_json::to_string_pretty(&save_state) {
            Ok(json) => json,

            Err(err) => {
                eprintln!("序列化存档失败: {err}");
                return;
            }
        };

        // 获取用户目录
        let mut path = match dirs::home_dir() {
            Some(path) => path,
            None => {
                eprintln!("无法获取用户目录");
                return;
            }
        };

        // ~/.ajax
        path.push(constant_str::SAVE_FOLDER);
        // 创建目录
        if let Err(err) = fs::create_dir_all(&path) {
            eprintln!("创建存档目录失败: {err}");
            return;
        }

        // ~/.ajax/user_data.json
        path.push(constant_str::SAVE_FILE);

        // 写文件
        if let Err(err) = fs::write(&path, json) {
            eprintln!("存档失败: {err}");
            return;
        }

        println!("Save Success 游戏已保存!");
    }

    // 加载的话 应该是 游戏刚开始的时候 
    pub fn load(&self)
    {

    }



    // 开始游戏
    fn _playing(&mut self) {
        print!("> ");
        if let Err(error) = io::stdout().flush() {
            eprintln!("数据刷新失败! {}", error);
        }
        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            eprintln!("读取输入失败: {error}");
        }
        // let command = parse_command(&input);
        let command = parser(&input);
        router(command, self);
    }

    // 菜单模式
    fn _mainmenu(&mut self) {
        println!(
            r#"
========================
Ajax(made by LosAngelous)
========================
1. 开始游戏
2. 继续游戏
3. 游戏设置
4. 游戏成就
5. 退出游戏
========================
"#
        );

        print!("请选择: ");

        if let Err(error) = io::stdout().flush() {
            eprintln!("数据刷新失败: {error}");
            self.l_game_state = GameState::Exit;
            return;
        }

        let mut input = String::new();

        if let Err(error) = io::stdin().read_line(&mut input) {
            eprintln!("读取输入失败: {error}");
            self.l_game_state = GameState::Exit;
            return;
        }

        match input.trim() {
            "1" => {
                self.l_game_state = GameState::Playing;
            }
            "2" => {
                self.l_game_state = GameState::Settings;
            }
            "3" => {
                println!("成就系统暂未实现");
            }
            "4" => {
                self.l_game_state = GameState::Exit;
            }
            _ => {
                println!("无效选项，请输入 1、2、3 或 4");
            }
        }
    }

    fn _settings(&mut self) {}

    fn _loading(&mut self) {}
}
