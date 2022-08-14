use std::f32::consts::PI;

use bevy::prelude::*;

use crate::events::ball::SetBallEvent;

#[derive(Clone, Copy, Debug)]
pub enum BallType {
    Normal,
}
impl BallType {
    pub fn weight(&self) -> f32 {
        match *self {
            BallType::Normal => 1.0,
        }
    }
    pub fn density(&self) -> f32 {
        self.weight() / self.volume()
    }
    /// 二次元体積（面積）を返す
    pub fn volume(&self) -> f32 {
        self.radius() * self.radius() * PI
    }
    pub fn radius(&self) -> f32 {
        match *self {
            BallType::Normal => 20.0,
        }
    }
    /// ボールの反発係数. 2つをかけ合わせたものを衝突の際の反発係数として使う
    pub fn restitution(&self) -> f32 {
        match *self {
            BallType::Normal => 1.0,
        }
    }
    pub fn color(&self) -> Color {
        match *self {
            BallType::Normal => Color::BLUE,
        }
    }
}
pub trait SetBall {
    fn set_balls(&mut self, ball_type: BallType, num: u32) -> &mut Self;
}
impl SetBall for Vec<SetBallEvent> {
    fn set_balls(&mut self, ball_type: BallType, num: u32) -> &mut Self {
        for _ in 0..num {
            self.push(SetBallEvent { ball_type })
        }
        self
    }
}

/// 待機状態のボールを表す
#[derive(Component)]
pub struct BallNocking;

#[derive(Component)]
pub struct Ball {
    pub ball_type: BallType,
}
impl Default for Ball {
    fn default() -> Self {
        Ball {
            ball_type: BallType::Normal,
        }
    }
}
impl Ball {
    pub fn new(ball_type: BallType) -> Self {
        Self { ball_type }
    }
}

#[derive(Component)]
/// 獲得スコアを受け渡す
pub struct GoalinBall(pub u32);
