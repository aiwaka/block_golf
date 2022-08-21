use bevy::prelude::*;

use super::utils::{EdgeDirection, ToEdgeDirection};

#[derive(Component, Clone, Debug)]
pub struct Magnet {
    pub active: bool,
    pub direction: EdgeDirection,
    /// 磁石表面の磁束密度. 現状「強さ」程度の雑な使い方をする.
    pub flux_density: f32,
}
impl Magnet {
    /// 0: up, 1: down, 2: left, 3: rightでも指定できる.
    pub fn new(default_active: bool, direction: impl ToEdgeDirection, flux_density: f32) -> Self {
        Magnet {
            active: default_active,
            direction: direction.to_edge_direction(),
            flux_density,
        }
    }
}
