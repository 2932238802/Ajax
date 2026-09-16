// 时间
// 能够随着系统时间的流逝而流逝 现实生活中 1 秒钟 等于 游戏时间的  2分钟 （也是24小时）
// 然后 比如一些耗时动作 开采一个矿物 休息片刻 钓鱼 这些我又不希望 真的阻塞
// 比如休息半小时 也就是现实中15秒(也就是游戏里面的半小时) 15分钟的等待显然是不太好的游戏体验。 这里只需要播放一个打印动画 就可以了(暂停 系统流逝 但是 播放完毕了 加上这个时长)
// 用一个任务队列来实现，比如现实每经过 5 秒钟 给 任务队列 推入 一个 tick(也就是游戏中10分钟)
// 当要决定一个耗时任务的时候，马上推入一个不完整的 tick(也有可能完整，也就是<=10 推入)
// 任务完成后，推入任务的tick(可能30分钟，可能20分钟)
// 然后 这个 队列 每次 取出tick的时候，更新游戏时间系统

use core::fmt;
use std::fmt::{Display, Formatter, Result};
use std::time::Instant;

struct LosTime {
    l_game_minutes: u64,    // 游戏进行的分钟 会 加时段 也会随着 系统进行更新
    l_last_update: Instant, // 上一次 更新的时间
    l_paused: bool,         // 现在是不是暂停的
}

// 
impl LosTime {
    // 初始化
    pub fn new() -> Self {
        Self {
            l_game_minutes: 0,
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
    pub fn update(&mut self) {
        if self.l_paused {
            return;
        }
        let now = Instant::now(); // 此刻
        let elaspe = now.duration_since(self.l_last_update);
        let seconds = elaspe.as_secs();
        if seconds > 0 {
            let minuted_elaspe = seconds * 2;
            self.l_game_minutes += minuted_elaspe;
            self.l_last_update += std::time::Duration::from_secs(seconds);
        }
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
    pub fn resumn(&mut self) {}
}

// 打印
impl Display for LosTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "天 {} 时 {} 分 {} | ",
            self.day(),
            self.hour(),
            self.minute()
        )
    }
}
