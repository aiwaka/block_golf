use bevy::prelude::*;

use crate::components::block::BlockSlidePath;

// 定義された軌道を実際に計算する.
// paramからVec2を返す. ブロックの中心を原点とする相対的なものにする.
// autowrapに対応して[-1, 1]を定義域とする関数の周期関数であると定める.
// manualでしか使わないのであればそうでなくてもよいがコンパイルの時点で制限されない.

pub fn calc_orbit(path: &BlockSlidePath, param: f32) -> Vec2 {
    match *path {
        BlockSlidePath::NoPath => Vec2::ZERO,
        BlockSlidePath::StandardLine { theta, width } => {
            Vec2::new(theta.cos(), theta.sin()) * width * param
        }
    }
}
