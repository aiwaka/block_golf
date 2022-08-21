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
    /// 回転軸の位置（軌道の影響で逐次更新される）
    pub rot_axis_pos: Vec2,
    /// 0度のときの「回転軸から」矩形中心までの相対位置
    /// rot_axis_posが0でposが(1,0)で角度が0度なら矩形中心は(1,0)にある.
    pub pos: Vec2,
    /// 角度
    pub angle: f32,
}

impl ToVolume for RectangleCollision {
    fn to_volume(&self) -> f32 {
        self.extents.x * self.extents.y
    }
}
impl ToAbsolutePos for RectangleCollision {
    fn to_absolute_pos(&self) -> Vec2 {
        self.rot_axis_pos + Vec2::from_angle(self.angle).rotate(self.pos)
    }
}
