use bevy::prelude::*;

use super::timer::CountDownTimer;

/// ゲームのルールを表す. グローバルなリソースとして使う.
#[derive(Clone, Copy, Debug)]
pub enum GameRule {
    /// 制限時間に入れたボールによるスコア
    BallScore,
    /// 1つ目のボールを入れるまでの操作時間が短いほど高スコア
    LittleOperation,
    /// 1つ目のボールを入れるまでの時間を競う
    TimeAttack,
}

pub struct GameOverEvent;

/// ゴールしたボール
pub struct GoaledBall(pub u32);
/// 最初のボールの総数
pub struct InitialBallNum(pub u32);

/// ゲーム全体から参照できるグローバルデータ（主にフラグ）
pub struct GlobalData {}
