use bevy::prelude::*;

use crate::{
    components::{
        ball::SetBallEvent,
        block::SpawnBlockEvent,
        game::{GoaledBall, InitialBallNum, OperationAmount, PassedTime, Score},
        goal::SpawnGoalEvent,
        info::RemainingTime,
        launcher::SpawnLauncherEvent,
        timer::CountDownTimer,
    },
    // stages::{debug::debug_stage, sample::sample_stage},
    stages::debug::debug_stage,
};

pub fn global_setup(mut commands: Commands) {
    // カメラのセット
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn stage_setup(
    mut commands: Commands,
    mut launcher_event_writer: EventWriter<SpawnLauncherEvent>,
    mut block_event_writer: EventWriter<SpawnBlockEvent>,
    mut goal_event_writer: EventWriter<SpawnGoalEvent>,
    mut ball_event_writer: EventWriter<SetBallEvent>,
) {
    commands.insert_resource(GoaledBall(0));
    commands.insert_resource(Score(0));
    commands.insert_resource(OperationAmount(0));
    commands.insert_resource(PassedTime(0));

    info!("stage setup");
    let stage_info = debug_stage();
    // let stage_info = sample_stage();
    let launcher_info = stage_info.launcher;
    let block_list = stage_info.blocks;
    let goal_list = stage_info.goal_pos;
    let ball_list = stage_info.balls;
    commands.insert_resource(InitialBallNum(ball_list.len() as u32));

    // 残り時間タイマー用意
    commands
        .spawn()
        .insert(RemainingTime)
        .insert(CountDownTimer::new(stage_info.time));

    launcher_event_writer.send(launcher_info.to_spawn_event());

    for block in block_list {
        // block_event_writer.send(block.to_spawn_event())
        block_event_writer.send(SpawnBlockEvent::from(&block));
    }
    for ball in ball_list {
        ball_event_writer.send(ball.to_spawn_event())
    }
    for goal in goal_list {
        goal_event_writer.send(goal.to_spawn_event())
    }
}
