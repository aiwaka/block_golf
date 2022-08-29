use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Component, Clone, Copy, Default, Debug)]
pub struct OffsetByUpdater;
#[derive(Component, Clone, Copy, Default, Debug)]
pub struct AngleByUpdater;

#[derive(Component, Clone, Debug)]
pub struct Updater {
    /// スイッチ等のidを保存する
    pub source_id: u32,
    /// キューからマイフレーム値を取り出して使用させる
    pub count_queue: VecDeque<u32>,
    /// deleteカウントで消去
    pub delete_count: u32,
    /// 最後に使ったカウントを保持する
    pub prev_count: u32,
}
impl Updater {
    pub fn new(source_id: u32, count_vec: Vec<u32>, delete_count: u32) -> Self {
        Self {
            source_id,
            count_queue: VecDeque::from(count_vec),
            delete_count,
            prev_count: 0,
        }
    }
}

#[derive(Component, Clone, Debug)]
pub struct BlockPosUpdater {
    pub func: fn(u32) -> Vec2,
}
#[derive(Component, Clone, Debug)]
pub struct BlockAngleUpdater(fn(i32) -> f32);

// /// クロージャを保持して何らかの変更を行う
// #[derive(Component, Clone, Debug)]
// pub enum UpdaterType {
//     #[allow(dead_code)]
//     /// 空選択肢
//     None,
//     BlockPos {
//         /// 位置決定関数
//         func: fn(i32) -> Vec2,
//     },
//     BlockAngle {
//         /// 角度決定関数
//         func: fn(i32) -> f32,
//     },
// }

// #[derive(Clone, Debug)]
// pub struct Updater {
//     /// rangeをコピーして初期化され, 処理ごとに一つ取り出して計算する. 空になれば削除する.
//     pub current_range: Vec<i32>,
//     /// i32の列で範囲指定する
//     pub range: Vec<i32>,
//     pub updater_type: UpdaterType,
// }
// impl Updater {
//     pub fn new(range: Vec<i32>, updater_type: UpdaterType) -> Self {
//         // current_rangeはpopで取り出すので逆向きにしてセットする
//         let mut reversed_range = range.clone();
//         reversed_range.reverse();
//         Updater {
//             current_range: reversed_range,
//             range,
//             updater_type,
//         }
//     }
// }

// /// Updaterの列をコンポーネントとして付与することで同時に様々な変更ができる
// #[derive(Component, Clone, Debug)]
// pub struct UpdaterVec(pub Vec<Updater>);
// impl UpdaterVec {
//     pub fn new_from_a_updater(updater: Updater) -> Self {
//         UpdaterVec(vec![updater])
//     }
// }
