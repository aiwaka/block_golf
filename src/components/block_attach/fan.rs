use bevy::prelude::*;

use super::utils::{EdgeDirection, ToEdgeDirection};

/// 風の視覚エフェクト.
#[derive(Component, Clone, Debug)]
pub struct WindVisualEffect;

/// 送風機は一定形状のブロックと同じとする.
/// 当たり判定を拡張するにはブロックと重ねる.
#[derive(Component, Clone, Debug)]
pub struct Fan {
    pub active: bool,
    pub direction: EdgeDirection,
    // ボールの断面積により受ける力が変わる
    pub pressure: f32,
}
impl Fan {
    /// 0: up, 1: down, 2: left, 3: rightでも指定できる.
    pub fn new(default_active: bool, direction: impl ToEdgeDirection, pressure: f32) -> Self {
        Fan {
            active: default_active,
            direction: direction.to_edge_direction(),
            pressure,
        }
    }
}
