use bevy::prelude::*;

/// 変更があって更新を行っていることを表す
#[derive(Component)]
pub struct MagazineUpdating;

#[derive(Component)]
pub struct RemainingBall;

/// 今使用されて消えるボール（消えるエフェクト用）
#[derive(Component)]
pub struct ConsumingBall;
