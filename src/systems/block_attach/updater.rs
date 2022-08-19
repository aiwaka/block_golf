use bevy::prelude::*;

use crate::{
    components::{
        block::BlockTransform,
        block_attach::updater::{UpdaterType, UpdaterVec},
    },
    AppState,
};

fn update(mut block_query: Query<(&mut BlockTransform, &mut UpdaterVec)>) {
    for (mut block_trans, mut updater_vec) in block_query.iter_mut() {
        for updater in updater_vec.0.iter_mut() {
            match updater.updater_type {
                UpdaterType::BlockPos { func } => {
                    block_trans.offset = func(updater.count);
                }
                UpdaterType::BlockAngle { func } => {
                    block_trans.angle = func(updater.count);
                }
            }
            updater.count += 1;
        }
    }
}

/// countがlimitを超えたupdaterを取り除く.
/// すべてのupdaterが終了していた場合それ自体を取り除く.
fn auto_remove(mut commands: Commands, mut updater_query: Query<(&mut UpdaterVec, Entity)>) {
    for (mut updater_vec, ent) in updater_query.iter_mut() {
        updater_vec.0.retain(|u| u.count < u.limit);
        if updater_vec.0.is_empty() {
            commands.entity(ent).remove::<UpdaterVec>();
        }
    }
}

pub struct UpdaterPlugin;
impl Plugin for UpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(update.before("updater:auto_remove")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(auto_remove.label("updater:auto_remove")),
        );
    }
}
