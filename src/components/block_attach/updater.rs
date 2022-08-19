use bevy::prelude::*;

/// クロージャを保持して何らかの変更を行う
#[derive(Clone, Debug)]
pub enum UpdaterType {
    /// 空選択肢
    None,
    BlockPos {
        /// 位置決定関数
        func: fn(i32) -> Vec2,
    },
    BlockAngle {
        /// 角度決定関数
        func: fn(i32) -> f32,
    },
}

#[derive(Clone, Debug)]
pub struct Updater {
    /// countから1フレームごとに1増加し, limitを超えたら自動で除去される.
    /// 負の値から初めて一定時間動作しないようなupdaterも設定可能
    pub count: i32,
    pub limit: i32,
    pub updater_type: UpdaterType,
}
impl Default for Updater {
    fn default() -> Self {
        Updater {
            count: 0,
            limit: 60,
            updater_type: UpdaterType::None,
        }
    }
}

/// Updaterの列をコンポーネントとして付与することで同時に様々な変更ができる
#[derive(Component, Clone, Debug)]
pub struct UpdaterVec(pub Vec<Updater>);
impl UpdaterVec {
    pub fn new_from_a_updater(updater: Updater) -> Self {
        UpdaterVec(vec![updater])
    }
}
