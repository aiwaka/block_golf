use crate::{
    components::physics::{
        acceleration::Acceleration,
        force::Force,
        material::{PhysicMaterial, Volume},
        position::Position,
        velocity::Velocity,
    },
    AppState,
};
use bevy::prelude::*;

/// 各フレームでボールにはたらく力による影響を計算する.
/// 力が加わっていれば加速度をセットし, 加わっていなければ加速度を0にする
pub fn execute_force(
    mut q: Query<(
        Option<&mut Force>,
        &mut Acceleration,
        &PhysicMaterial,
        &Volume,
    )>,
) {
    for (f, mut a, material, vol) in q.iter_mut() {
        if let Some(mut f) = f {
            a.0 = f.0 / material.density / vol.0;
            f.0 = Vec2::ZERO;
        } else {
            a.0 = Vec2::ZERO;
        };
    }
}

pub fn accelerate(mut q: Query<(&Acceleration, &mut Velocity)>) {
    for (a, mut v) in q.iter_mut() {
        v.0 += a.0;
    }
}

pub fn move_position(mut q: Query<(&Velocity, &mut Position)>) {
    for (v, mut p) in q.iter_mut() {
        p.0 += v.0;
    }
}

pub(super) struct MotionDynamicsPlugin;
impl Plugin for MotionDynamicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(execute_force.label("execute_force")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(accelerate.after("execute_force").label("accelerate")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(move_position.after("accelerate").label("move_pos")),
        );
    }
}
