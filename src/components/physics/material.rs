use std::f32::consts::PI;

use bevy::prelude::Component;
use bevy_prototype_lyon::shapes::{Circle, Rectangle};

#[derive(Component, Debug, Clone, Copy)]
pub struct PhysicMaterial {
    pub restitution: f32, // 反発係数
    pub density: f32,     // 密度
    pub friction: f32,    // 摩擦係数
}
impl Default for PhysicMaterial {
    fn default() -> Self {
        PhysicMaterial {
            restitution: 1.0,
            density: 1.0,
            friction: 0.0,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Volume(pub f32);

pub trait ToVolume {
    fn to_volume(&self) -> f32;
}
impl ToVolume for f32 {
    fn to_volume(&self) -> f32 {
        *self
    }
}
impl ToVolume for Rectangle {
    fn to_volume(&self) -> f32 {
        self.extents.x * self.extents.y
    }
}
impl ToVolume for Circle {
    fn to_volume(&self) -> f32 {
        self.radius * self.radius * PI
    }
}

impl PhysicMaterial {
    pub fn new(restitution: f32, density: f32, friction: f32) -> Self {
        Self {
            restitution,
            density,
            friction,
        }
    }
}
