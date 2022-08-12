use bevy::prelude::*;

/// ボールが踏んで何らかの効果を発動させるスイッチ
#[derive(Component)]
pub struct SwitchTile {
    /// 押された瞬間のみtrueになるフラグ
    pub just_active: bool,
    /// 押下中フラグ
    pub active: bool,
    /// 発動する重さ制限（必ず発動するなら0にすればよい）
    pub threshold: f32,
    /// 押された後自動で解除されるかどうか（フレーム数のOptionで指定）
    pub auto_reverse: Option<u32>,
    /// SwitchTargetに付けた整数を指定して効果対象を限定する.
    pub target_id: u32,
}

#[derive(Component)]
pub struct SwitchTarget {
    pub target_id: u32,
}
