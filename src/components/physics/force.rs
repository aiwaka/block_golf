use bevy::prelude::*;

#[derive(Component, Debug, Clone)]
pub struct Force(pub Vec2);

#[derive(Component, Debug, Clone)]
pub struct Gravity {
    /// ベクトル場を重力加速度場として考える
    pub field: fn(Vec2) -> Vec2,
}
impl Gravity {
    /// 重力なし
    pub fn no_gravity() -> Self {
        Gravity {
            field: |_: Vec2| Vec2::ZERO,
        }
    }
    /// 下向きの一様重力場
    pub fn simple_gravity() -> Self {
        Gravity {
            field: |_: Vec2| Vec2::new(0.0, -0.5),
        }
    }

    pub fn new(field: fn(Vec2) -> Vec2) -> Self {
        Self { field }
    }
    pub fn new_as_some(field: fn(Vec2) -> Vec2) -> Option<Self> {
        Some(Gravity::new(field))
    }
}
