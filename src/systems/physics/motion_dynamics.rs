use crate::{
    components::{
        ball::Ball,
        physics::{
            acceleration::Acceleration, force::Force, material::PhysicMaterial, position::Position,
            velocity::Velocity,
        },
    },
    AppState,
};
use bevy::prelude::*;

/// ボールにはたらく力による影響を計算する.
/// 力が加わっていれば加速度をセットし, 加わっていなければ加速度を0にする
/// 現状ボールしか使えないようになっている.
/// TODO: lyonのShapeBundleから体積を計算できないか
pub fn execute_force(
    mut commands: Commands,
    mut q: Query<(
        Option<&Force>,
        &mut Acceleration,
        &PhysicMaterial,
        &Ball,
        Entity,
    )>,
) {
    for (f, mut a, _, ball, ent) in q.iter_mut() {
        if let Some(f) = f {
            a.0 = f.0 / ball.ball_type.weight();
            commands.entity(ent).remove::<Force>();
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

pub struct MotionDynamicsPlugin;
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
