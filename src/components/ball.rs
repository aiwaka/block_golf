use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum BallType {
    Normal,
}

/// 待機状態のボールを表す
#[derive(Component)]
pub struct BallNocking;

#[derive(Component)]
pub struct Ball {
    pub direction: Vec2, // 絶対値をspeedとする
    pub ball_type: BallType,
}
impl Default for Ball {
    fn default() -> Self {
        Ball {
            direction: Vec2::new(4.0, 0.0),
            ball_type: BallType::Normal,
        }
    }
}
impl Ball {
    pub fn new(direction: Vec2, ball_type: BallType) -> Self {
        Self {
            direction,
            ball_type,
        }
    }
}

// ボールを出現させる. 待機状態になる.
pub struct SpawnBallEvent {
    pub ball_type: BallType,
}
// 実際に発射する.
pub struct LaunchBallEvent {
    pub direction: Vec2,
}
