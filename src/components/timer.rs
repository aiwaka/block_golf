use bevy::prelude::*;

/// 初期化したフレーム数減少するカウントダウン
#[derive(Component)]
pub struct CountDownTimer {
    count: u32,
    pause: bool,
    pub auto_despawn: bool,
}
impl Default for CountDownTimer {
    fn default() -> Self {
        CountDownTimer {
            count: 0,
            pause: false,
            auto_despawn: true,
        }
    }
}
impl CountDownTimer {
    pub fn new(count: u32) -> Self {
        Self {
            count,
            ..Default::default()
        }
    }
    /// 自動でエンティティを削除されないコンポーネントとして作成する（このコンポーネント自体は終了時取り除かれる）
    pub fn new_will_not_be_removed(count: u32) -> Self {
        Self {
            count,
            auto_despawn: false,
            ..Default::default()
        }
    }
    pub fn tick(&mut self) {
        if !self.pause && self.count > 0 {
            self.count -= 1;
        }
    }
    pub fn count(&self) -> u32 {
        self.count
    }
    pub fn is_finished(&self) -> bool {
        self.count == 0
    }
    pub fn stop(&mut self) {
        self.pause = true;
    }
    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }
}

/// tickを呼ぶと1増加するカウンター
#[derive(Component)]
pub struct FrameCounter {
    pub count: u32,
    pub pause: bool,
}
impl FrameCounter {
    pub fn new() -> Self {
        Self {
            count: 0,
            pause: false,
        }
    }
    pub fn tick(&mut self) {
        if !self.pause {
            self.count += 1;
        }
    }
    pub fn reset(&mut self) {
        self.count = 0;
    }
    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }
}
