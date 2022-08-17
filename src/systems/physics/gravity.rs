use bevy::prelude::*;

use crate::{
    components::physics::{
        force::{Force, Gravity},
        material::{PhysicMaterial, Volume},
        position::Position,
    },
    AppState,
};

/// 重力を物体に加える
fn gravity_effect(
    mut query: Query<(&Position, &mut Force, &PhysicMaterial, &Volume)>,
    gravity_query: Query<&Gravity>,
) {
    if let Ok(gravity) = gravity_query.get_single() {
        for (pos, mut force, mat, vol) in query.iter_mut() {
            force.0 += (gravity.field)(pos.0) * vol.0 * mat.density;
        }
    }
}

pub struct GravityPlugin;
impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(gravity_effect));
    }
}
