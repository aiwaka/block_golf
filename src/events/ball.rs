use crate::components::ball::BallType;
use bevy::prelude::*;

// ステージ情報とランチャーを受け渡すイベント
pub struct SetBallEvent {
    pub ball_type: BallType,
}

// ボールを出現させる. 待機状態になる.
pub struct SpawnBallEvent {
    pub ball_type: BallType,
    pub pos: Vec2,
}
// 実際に発射する.
pub struct LaunchBallEvent {
    pub direction: Vec2,
}
