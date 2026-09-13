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
        map::map::LosMap,
        save::data::{SaveData, SaveMap, SavePlayer, SaveState},
    },
    entity::player::LosPlayer,
};

use colored::*;

// 主要游戏界面
pub struct LosGame {
    pub l_world: LosWorld,
    pub l_player: LosEntity,
    l_running: bool,
    l_has_save_data: bool, // 是不是有存档
    l_game_state: GameState,
}

// 输入输出循环
// 先尝试加载
impl LosGame {
    // 生成游戏
    pub fn new() -> Self {
        match Self::load() {
            Ok(game) => game,
            Err(err_string) => {
                let mut world = LosWorld::new();
                let player = LosPlayer::spawn(&mut world);
                Self {
                    l_world: world,
                    l_player: player,
                    l_running: true,
                    l_has_save_data: false,
                    l_game_state: GameState::MainMenu,
                }
            }
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
    // 成功是 () 失败是 String
    pub fn load() -> Result<Self, String> {
        let mut path = dirs::home_dir().ok_or("读取存储失败")?;
        path.push(constant_str::SAVE_FOLDER);
        path.push(constant_str::SAVE_FILE);
        if !path.exists() {
            return Err("保存的文件路径不存在".to_string());
        }
        let json = fs::read_to_string(&path).map_err(|e| format!("读取存档失败! {e}"))?;
        let save_data: SaveData =
            serde_json::from_str(&json).map_err(|e| format!("解析存档失败! {e}"))?;
        let map = LosMap::from_save_data(&save_data.l_map);
        let mut world = LosWorld::new();
        world.l_map = map;
        let player = LosPlayer::from_save_data(&save_data.l_player, &mut world);
        Ok(Self {
            l_world: world,
            l_player: player,
            l_running: true,
            l_has_save_data: true,
            l_game_state: GameState::MainMenu,
        })
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

    fn _mainmenu(&mut self) {
        let has_save = self.l_has_save_data;

        println!();
        println!("{}", "═══════════════════════════".cyan().bold());
        println!("         {}", "Ajax".bright_cyan().bold());
        println!("    {}", "made by LosAngelous".bright_black());
        println!("{}", "═══════════════════════════".cyan().bold());
        println!();

        println!(
            "  {}  {}",
            "1.".bright_white().bold(),
            "开始游戏".bright_green()
        );

        if has_save {
            println!(
                "  {}  {}",
                "2.".bright_white().bold(),
                "继续游戏".bright_yellow()
            );
        } else {
            println!(
                "  {}  {} {}",
                "2.".bright_black(),
                "继续游戏".bright_black(),
                "(无存档)".truecolor(80, 80, 80)
            );
        }

        println!(
            "  {}  {}",
            "3.".bright_white().bold(),
            "游戏设置".bright_blue()
        );

        println!(
            "  {}  {}",
            "4.".bright_white().bold(),
            "游戏成就".bright_magenta()
        );

        println!(
            "  {}  {}",
            "5.".bright_white().bold(),
            "退出游戏".bright_red()
        );

        println!();
        println!("{}", "═══════════════════════════".cyan().bold());
        println!();

        print!("{} ", "请选择:".bright_white());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            eprintln!("{}", format!("读取输入失败: {error}").red());
            self.l_game_state = GameState::Exit;
            return;
        }

        match input.trim() {
            "1" => {
                println!("{}", "→ 开始新游戏".green());

                // 创建新游戏
                let mut world = LosWorld::new();
                let player = LosPlayer::spawn(&mut world);

                self.l_world = world;
                self.l_player = player;
                self.l_game_state = GameState::Playing;
            }

            "2" => {
                if has_save {
                    println!("{}", "→ 加载存档中...".yellow());

                    match Self::load() {
                        Ok(loaded_game) => {
                            *self = loaded_game;
                            self.l_game_state = GameState::Playing;
                            println!("{}", "✓ 存档加载成功".green());
                        }
                        Err(e) => {
                            eprintln!("{}", format!("✗ 加载失败: {e}").red());
                        }
                    }
                } else {
                    println!("{}", "✗ 没有存档，请先开始新游戏".yellow());
                }
            }
            "3" => {
                println!("{}", "→ 进入设置".blue());
                self.l_game_state = GameState::Settings;
            }
            "4" => {
                println!("{}", "→ 查看成就".magenta());
                println!("{}", "成就系统开发中...".yellow());
            }
            "5" => {
                println!("{}", "→ 正在退出...".red());
                self.save();
                self.l_game_state = GameState::Exit;
            }
            _ => {
                println!("{}", format!("✗ 无效选项: '{}'", input.trim()).red());
            }
        }
    }

    fn _settings(&mut self) {}

    fn _loading(&mut self) {}
}
