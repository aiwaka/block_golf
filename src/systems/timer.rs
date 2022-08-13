use bevy::prelude::*;

use crate::components::timer::{CountDownTimer, FrameCounter};

pub fn count_down_update(mut query: Query<&mut CountDownTimer>) {
    for mut timer in query.iter_mut() {
        timer.tick();
    }
}
/// 使い終わったタイマーは自動で削除される.
/// 通常のSystemが登録されるUpdateStageがすべて処理されたあとに処理される.
/// タイマーが終わったことを使うSystemには.after("count_down_update")の指定が必要になる.
fn delete_counter(mut commands: Commands, query: Query<(&CountDownTimer, Entity)>) {
    for (timer, ent) in query.iter() {
        if timer.is_finished() {
            if timer.auto_despawn {
                commands.entity(ent).despawn();
            } else {
                commands.entity(ent).remove::<CountDownTimer>();
            }
        }
    }
}

fn frame_counter_update(mut query: Query<&mut FrameCounter>) {
    for mut counter in query.iter_mut() {
        counter.tick();
    }
}

pub struct TimersPlugin;
impl Plugin for TimersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(count_down_update.label("count_down_update"));
        app.add_system(frame_counter_update.label("frame_counter_update"));
        app.add_system_to_stage(CoreStage::Last, delete_counter);
    }
}
