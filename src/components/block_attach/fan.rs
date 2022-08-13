use bevy::prelude::*;

pub struct SpawnFanEvent {
    pub fan_obj: Fan,
}

/// 送風機を四角形ブロックに取り付けるときの辺の方向
#[derive(Debug, Clone)]
pub enum FanDirection {
    Up,
    Down,
    Left,
    Right,
}

/// 送風機は一定形状のブロックと同じとする.
/// 当たり判定を拡張するにはブロックと重ねる.
#[derive(Component, Clone, Debug)]
pub struct Fan {
    pub active: bool,
    pub direction: FanDirection,
    // ボールの断面積により受ける力が変わる
    pub pressure: f32,
}
