use crate::{
    components::physics::{acceleration::Acceleration, position::Position, velocity::Velocity},
    AppState,
};
use bevy::prelude::*;

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
            SystemSet::on_update(AppState::Game).with_system(accelerate.label("accelerate")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(move_position.after("accelerate").label("move_pos")),
        );
        // app.add_system(accelerate.label("accelerate"));
        // app.add_system(move_position.after("accelerate").label("move_pos"));
    }
}
