pub mod acceleration;
pub mod force;
pub mod material;
pub mod position;
pub mod velocity;

use bevy::prelude::*;

use self::{
    acceleration::Acceleration,
    force::Force,
    material::{PhysicMaterial, ToVolume, Volume},
    position::Position,
    velocity::Velocity,
};

/// 基本的な物理演算用の量をまとめたもの.
/// 初期にForceを加えられるようにするかどうかは要検討
#[derive(Bundle, Debug, Clone)]
pub struct BasicPhysicsBundle {
    pos: Position,
    vel: Velocity,
    acc: Acceleration,
    force: Force,
    material: PhysicMaterial,
    vol: Volume,
}
impl BasicPhysicsBundle {
    pub fn new(
        pos: Vec2,
        vel: Vec2,
        acc: Vec2,
        material: PhysicMaterial,
        vol: &impl ToVolume,
    ) -> Self {
        Self {
            pos: Position(pos),
            vel: Velocity(vel),
            acc: Acceleration(acc),
            force: Force(Vec2::ZERO),
            material,
            vol: Volume(vol.to_volume()),
        }
    }
}

#[derive(Bundle, Debug, Clone)]
pub struct MaterialPhysicsBundle {}
impl MaterialPhysicsBundle {
    pub fn new(material: PhysicMaterial, vol: f32) -> Self {
        Self {}
    }
}
