use bevy::prelude::*;

use crate::{
    components::{
        block::BlockTransformInfo,
        block_attach::updater::{BlockPosUpdater, Updater},
        timer::FrameCounter,
    },
    AppState,
};

fn block_pos_update(
    mut block_query: Query<(&mut BlockTransformInfo, &Children)>,
    updater_q: Query<(&Updater, &BlockPosUpdater, &FrameCounter)>,
) {
    for (mut block_trans_info, mut block_children) in block_query.iter_mut() {
        for &child in block_children.iter() {
            if let Ok((updater, pos_updater, counter)) = updater_q.get(child) {
                if updater.range.contains(&counter.count) {
                    block_trans_info.offset += (pos_updater.func)(**counter);
                }
            }
        }
    }
}

/// countがrangeを超えたupdaterを取り除く.
fn auto_remove(mut commands: Commands, mut updater_q: Query<(&Updater, &FrameCounter, Entity)>) {
    for (updater, counter, ent) in updater_q.iter() {
        if !updater.range.contains(&counter.count) {
            commands.entity(ent).despawn();
        }
    }
}

pub struct UpdaterPlugin;
impl Plugin for UpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(block_pos_update.before("updater:auto_remove")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(auto_remove.label("updater:auto_remove")),
        );
    }
}
