use bevy::prelude::*;

use crate::components::timer::{CountDownTimer, FrameCounter};

pub fn count_down(mut query: Query<&mut CountDownTimer>) {
    for mut timer in query.iter_mut() {
        timer.tick();
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
        app.add_system(count_down);
        app.add_system(frame_counter_update);
    }
}
