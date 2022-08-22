use bevy::prelude::*;

use super::physics::material::ToVolume;

/// 絶対位置に変換できるトレイト
pub trait ToAbsolutePos {
    fn to_absolute_pos(&self) -> Vec2;
}
#[derive(Component, Clone, Debug)]
pub struct RectangleCollision {
    /// 矩形の大きさ
    pub extents: Vec2,
    /// 位置（自動で更新する）
    pub pos: Vec2,
    /// 角度（自動で更新する）
    pub angle: f32,
    /// 前フレームの位置
    pub prev_pos: Vec2,
    pub prev_angle: f32,
}
impl RectangleCollision {
    pub fn new(extents: Vec2) -> Self {
        Self {
            extents,
            pos: Vec2::ZERO,
            angle: 0.0,
            prev_pos: Vec2::ZERO,
            prev_angle: 0.0,
        }
    }
}

impl ToVolume for RectangleCollision {
    fn to_volume(&self) -> f32 {
        self.extents.x * self.extents.y
    }
}
impl ToAbsolutePos for RectangleCollision {
    fn to_absolute_pos(&self) -> Vec2 {
        self.pos
    }
}
