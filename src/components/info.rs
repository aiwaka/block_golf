use bevy::prelude::*;

#[derive(Component)]
pub struct RemainingTime;

/// 変更があって更新を行っていることを表す
#[derive(Component)]
pub struct MagazineUpdating;

#[derive(Component)]
pub struct RemainingBall;

/// 今使用されて消えるボール（消えるエフェクト用）
#[derive(Component)]
pub struct ConsumingBall;

/// 結果表示系のコンポーネント
#[derive(Component)]
pub struct WaitForResultDisplay;
#[derive(Component)]
pub struct ResultText;
