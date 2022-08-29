use bevy::prelude::*;

use crate::{
    components::{
        block::{BlockAngle, BlockPosOffset},
        block_attach::updater::{
            AngleByUpdater, BlockAngleUpdater, BlockPosUpdater, OffsetByUpdater, Updater,
        },
        timer::FrameCounter,
    },
    AppState,
};

fn block_pos_update(
    mut block_query: Query<&Children>,
    mut updater_q: Query<
        (&mut Updater, &BlockPosUpdater, &mut BlockPosOffset),
        With<OffsetByUpdater>,
    >,
) {
    for block_children in block_query.iter_mut() {
        for &child in block_children.iter() {
            if let Ok((mut updater, pos_updater, mut block_offset)) = updater_q.get_mut(child) {
                if let Some(next_count) = updater.count_queue.pop_front() {
                    block_offset.0 = (pos_updater.func)(next_count);
                }
            }
        }
    }
}

fn block_angle_update(
    mut block_query: Query<&Children>,
    mut updater_q: Query<(&mut Updater, &BlockAngleUpdater, &mut BlockAngle), With<AngleByUpdater>>,
) {
    for block_children in block_query.iter_mut() {
        for &child in block_children.iter() {
            if let Ok((mut updater, angle_updater, mut block_angle)) = updater_q.get_mut(child) {
                if let Some(next_count) = updater.count_queue.pop_front() {
                    block_angle.0 = (angle_updater.func)(next_count);
                }
            }
        }
    }
}

/// countがrangeを超えたupdaterを取り除く.
fn auto_remove(mut commands: Commands, updater_q: Query<(&Updater, &FrameCounter, Entity)>) {
    for (updater, counter, ent) in updater_q.iter() {
        if counter.count > updater.delete_count {
            commands.entity(ent).despawn();
        }
    }
}

pub struct UpdaterPlugin;
impl Plugin for UpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(
                block_pos_update
                    .before("block:reflect_transform_and_collision")
                    .after("updater:auto_remove"),
            ),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(
                block_angle_update
                    .before("block:reflect_transform_and_collision")
                    .after("updater:auto_remove"),
            ),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(
                auto_remove
                    .after("frame_counter_update")
                    .label("updater:auto_remove"),
            ),
        );
    }
}
