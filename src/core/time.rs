// 时间
// 现实时间会推动游戏时间流逝，比例由 DEFAULT_TIME_MULTY 决定（当前 1 秒 ≈ 0.8 游戏分钟）
// 然后 比如一些耗时动作 开采一个矿物 休息片刻 钓鱼 这些我又不希望 真的阻塞
// 比如休息半小时 也就是现实中15秒(也就是游戏里面的半小时) 15分钟的等待显然是不太好的游戏体验。 这里只需要播放一个打印动画 就可以了(暂停 系统流逝 但是 播放完毕了 加上这个时长)
// 用一个任务队列来实现，比如现实每经过 5 秒钟 给 任务队列 推入 一个 tick(也就是游戏中10分钟)
// 当要决定一个耗时任务的时候，马上推入一个不完整的 tick(也有可能完整，也就是<=10 推入)
// 任务完成后 推入任务的tick(可能30分钟 可能20分钟)
// 然后 这个 队列 每次 取出tick的时候 更新游戏时间系统

use crate::constants::constant_class::ACTION_ANIMATION;
use crate::constants::constant_number::DEFAULT_TIME_MULTY;
use crate::core::data::SaveTime;
use core::fmt;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result};
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::{print, println, thread};

#[derive(Debug, Clone)]
pub struct LosTime {
    l_game_minutes: u64,    // 游戏进行的分钟 会 加时段 也会随着 系统进行更新
    l_minute_carry: f64,    // 累积不足 1 分钟的小数部分，避免整数截断导致时间停滞
    l_last_update: Instant, // 上一次 更新的时间
    l_paused: bool,         // 现在是不是暂停的
}

impl LosTime {
    // 初始化
    pub fn new() -> Self {
        Self {
            l_game_minutes: 0,
            l_minute_carry: 0.0,
            l_last_update: Instant::now(),
            l_paused: false,
        }
    }

    // 从 保存的数据 里面 拿出来
    pub fn from_save_data(minute: &SaveTime) -> Self {
        Self {
            l_game_minutes: minute.l_elasped,
            l_minute_carry: 0.0,
            l_last_update: Instant::now(),
            l_paused: false,
        }
    }

    // 返回流逝的分钟
    pub fn minute(&self) -> u64 {
        self.l_game_minutes % 60
    }

    // 当天的时间
    pub fn hour(&self) -> u64 {
        (self.l_game_minutes / 60) % 24
    }

    // 当天的时间
    pub fn day(&self) -> u64 {
        self.l_game_minutes / (60 * 24) + 1
    }

    // 获取 总的 分钟数
    pub fn total_minutes(&self) -> u64 {
        self.l_game_minutes
    }

    // 更新时间
    pub fn update(&mut self) -> f64 {
        if self.l_paused {
            return 0.0;
        }
        let now = Instant::now(); // 此刻
        let elapsed = now.duration_since(self.l_last_update);
        self.l_last_update = now;
        let minuted_elaspe = elapsed.as_secs_f64() * DEFAULT_TIME_MULTY;
        if minuted_elaspe > 0.0 {
            self.l_minute_carry += minuted_elaspe;
            let whole_minutes = self.l_minute_carry.trunc();
            self.l_game_minutes += whole_minutes as u64;
            self.l_minute_carry -= whole_minutes;
            // l_last_update 已在上面更新
        }
        minuted_elaspe
    }

    // 往前 推 时间
    pub fn advance_minutes(&mut self, minutes: u64) {
        self.l_game_minutes += minutes;
    }

    // 暂停
    pub fn pause(&mut self) {
        self.l_last_update = Instant::now();
        self.l_paused = true;
    }

    // 恢复
    pub fn resume(&mut self) {
        self.l_paused = false;
        self.l_last_update = Instant::now();
    }

    // 动作 名字 和 消耗时间
    pub fn do_action(&mut self, active_name: &str, cost_minutes: u64) {
        self.pause(); // 暂停
        let frames = ACTION_ANIMATION;
        for i in 0..15 {
            print!("\r{} {}", frames[i % frames.len()], active_name);
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        println!("\r{} 完成! finied: {} 分钟", active_name, cost_minutes);
        self.advance_minutes(cost_minutes);
        self.resume();
    }
}

// 打印
impl Display for LosTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "第 {} 天 {:02}:{:02}",
            self.day(),
            self.hour(),
            self.minute()
        )
    }
}
