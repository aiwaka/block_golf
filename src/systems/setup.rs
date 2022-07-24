use bevy::prelude::*;

use crate::{
    components::{ball::SetBallEvent, block::SpawnBlockEvent, goal::SpawnGoalEvent},
    stages::sample::sample_stage,
};

pub fn global_setup(mut commands: Commands) {
    // カメラのセット
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn stage_setup(
    mut block_event_writer: EventWriter<SpawnBlockEvent>,
    mut goal_event_writer: EventWriter<SpawnGoalEvent>,
    mut ball_event_writer: EventWriter<SetBallEvent>,
) {
    info!("stage setup");
    // let stage_info = debug_stage();
    let stage_info = sample_stage();
    let block_list = stage_info.blocks;
    let goal_list = stage_info.goal_pos;
    let ball_list = stage_info.balls;

    for block in block_list {
        block_event_writer.send(block)
    }
    for goal in goal_list {
        goal_event_writer.send(goal)
    }
    for ball in ball_list {
        ball_event_writer.send(ball)
    }
}
