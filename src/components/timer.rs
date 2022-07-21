use bevy::prelude::*;

/// 初期化したフレーム数減少するカウントダウン
#[derive(Component)]
pub struct CountDownTimer(pub u32);
impl CountDownTimer {
    pub fn new(count: u32) -> Self {
        Self(count)
    }
    pub fn tick(&mut self) {
        if self.0 > 0 {
            self.0 -= 1;
        }
    }
    pub fn is_finished(&self) -> bool {
        self.0 == 0
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
