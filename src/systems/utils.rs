//! システム内で使う汎用的な関数等を用意する

use crate::components::block_attach::utils::EdgeDirection;
use bevy::prelude::*;

// 方向を指定して矩形の両端点を計算する.
/// 反時計回りになるように2点を返す
pub fn calc_edge_points_of_rectangle(
    edge_direction: &EdgeDirection,
    block_orig_pos: Vec2,
    angle: f32,
    extents: Vec2,
) -> [Vec2; 2] {
    let half_ext = Vec2::from_angle(angle).rotate(extents / 2.0);
    // xだけ反転させたベクトル
    let refl_half_ext = Vec2::from_angle(angle).rotate(Vec2::new(-extents.x, extents.y) / 2.0);
    match edge_direction {
        EdgeDirection::Up => [block_orig_pos + half_ext, block_orig_pos + refl_half_ext],
        EdgeDirection::Down => [block_orig_pos - half_ext, block_orig_pos - refl_half_ext],
        EdgeDirection::Left => [block_orig_pos + refl_half_ext, block_orig_pos - half_ext],
        EdgeDirection::Right => [block_orig_pos - refl_half_ext, block_orig_pos + half_ext],
    }
}
