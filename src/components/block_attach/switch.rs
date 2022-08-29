use bevy::prelude::*;

use crate::components::block::{RotateStrategy, SlideStrategy};

/// ボールが踏んで何らかの効果を発動させるスイッチ
#[derive(Component, Clone, Debug)]
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
    /// 大きさ
    pub extents: Vec2,
}

/// 変更する内容ごとにここに登録する
#[derive(Clone, Debug)]
pub enum SwitchType {
    ChangeSlideStrategy {
        before: SlideStrategy,
        after: SlideStrategy,
    },
    ChangeRotateStrategy {
        before: RotateStrategy,
        after: RotateStrategy,
    },
    ToggleFanActive,
    MoveBlock {
        /// u32の列を保持してNoneが出るまで回す
        range: Vec<u32>,
        /// カウントから座標を計算する関数. BlockOriginalPosからの相対移動を行う.
        func: fn(u32) -> Vec2,
    },
    RotateBlock {
        /// u32の列を保持してNoneが出るまで回す
        range: Vec<u32>,
        /// カウントから座標を計算する関数. BlockOriginalPosからの相対移動を行う.
        func: fn(u32) -> f32,
    },
}

#[derive(Component, Clone, Debug)]
pub struct SwitchReceiver {
    pub switch_type: SwitchType,
    pub target_id: u32,
}
