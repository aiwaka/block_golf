use bevy::prelude::*;

#[derive(Clone, Copy)]
pub enum BallType {
    Normal,
}
impl BallType {
    pub fn weight(&self) -> f32 {
        match *self {
            BallType::Normal => 1.0,
        }
    }
    pub fn radius(&self) -> f32 {
        match *self {
            BallType::Normal => 20.0,
        }
    }
    /// ボール同士の反発係数. 組み合わせで記述
    pub fn ball_restitution(&self, other: &BallType) -> f32 {
        match (*self, other) {
            (BallType::Normal, BallType::Normal) => 1.0,
        }
    }
}

/// 待機状態のボールを表す
#[derive(Component)]
pub struct BallNocking;

#[derive(Component)]
pub struct Ball {
    pub pos: Vec2,
    pub direction: Vec2, // 絶対値をspeedとする
    pub ball_type: BallType,
}
impl Default for Ball {
    fn default() -> Self {
        Ball {
            pos: Vec2::ZERO,
            direction: Vec2::new(4.0, 0.0),
            ball_type: BallType::Normal,
        }
    }
}
impl Ball {
    pub fn new(pos: Vec2, direction: Vec2, ball_type: BallType) -> Self {
        Self {
            pos,
            direction,
            ball_type,
        }
    }
}

#[derive(Component)]
pub struct GoalinBall;

// ボールを出現させる. 待機状態になる.
pub struct SpawnBallEvent {
    pub ball_type: BallType,
}
// 実際に発射する.
pub struct LaunchBallEvent {
    pub direction: Vec2,
}
