use bevy::prelude::*;

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

/// ゲーム全体から参照できるグローバルデータ（主にフラグ）
pub struct GlobalData {}
