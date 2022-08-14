use bevy::prelude::*;

pub trait ToFanDirection {
    fn to_fan_direction(&self) -> FanDirection;
}
/// 送風機を四角形ブロックに取り付けるときの辺の方向
#[derive(Debug, Clone, Copy)]
pub enum FanDirection {
    Up,
    Down,
    Left,
    Right,
}
impl ToFanDirection for FanDirection {
    fn to_fan_direction(&self) -> FanDirection {
        *self
    }
}
impl ToFanDirection for u32 {
    fn to_fan_direction(&self) -> FanDirection {
        match self {
            0 => FanDirection::Up,
            1 => FanDirection::Down,
            2 => FanDirection::Left,
            _ => FanDirection::Right,
        }
    }
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
impl Fan {
    /// 0: up, 1: down, 2: left, 3: rightでも指定できる.
    pub fn new(default_active: bool, direction: impl ToFanDirection, pressure: f32) -> Self {
        Fan {
            active: default_active,
            direction: direction.to_fan_direction(),
            pressure,
        }
    }
}
