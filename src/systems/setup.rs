use bevy::prelude::*;

use crate::{
    components::{
        ball::SetBallEvent,
        block::SpawnBlockEvent,
        game::{GoaledBall, InitialBallNum, OperationAmount, PassedTime, Score},
        goal::SpawnGoalEvent,
        info::RemainingTime,
        timer::CountDownTimer,
    },
    stages::{debug::debug_stage, sample::sample_stage},
};

pub fn global_setup(mut commands: Commands) {
    // カメラのセット
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn stage_setup(
    mut commands: Commands,
    mut block_event_writer: EventWriter<SpawnBlockEvent>,
    mut goal_event_writer: EventWriter<SpawnGoalEvent>,
    mut ball_event_writer: EventWriter<SetBallEvent>,
) {
    commands.insert_resource(GoaledBall(0));
    commands.insert_resource(Score(0));
    commands.insert_resource(OperationAmount(0));
    commands.insert_resource(PassedTime(0));

    info!("stage setup");
    // let stage_info = debug_stage();
    let stage_info = sample_stage();
    let block_list = stage_info.blocks;
    let goal_list = stage_info.goal_pos;
    let ball_list = stage_info.balls;
    commands.insert_resource(InitialBallNum(ball_list.len() as u32));

    // 残り時間タイマー用意
    commands
        .spawn()
        .insert(RemainingTime)
        .insert(CountDownTimer::new(stage_info.time));

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
