use bevy::prelude::*;

use crate::{stages::structs::StageInfo, AppState};

use crate::components::{
    ball::SetBallEvent,
    block::SpawnBlockEvent,
    game::{GoaledBall, InitialBallNum, OperationAmount, PassedTime, Score},
    goal::SpawnGoalEvent,
    info::RemainingTime,
    launcher::SpawnLauncherEvent,
    timer::CountDownTimer,
};

pub fn stage_setup(
    mut commands: Commands,
    stage_info: Option<Res<StageInfo>>,
    mut launcher_event_writer: EventWriter<SpawnLauncherEvent>,
    mut block_event_writer: EventWriter<SpawnBlockEvent>,
    mut goal_event_writer: EventWriter<SpawnGoalEvent>,
    mut ball_event_writer: EventWriter<SetBallEvent>,
    mut app_state: ResMut<State<AppState>>,
) {
    // ステージ情報リソースが存在するならセットアップ開始
    if stage_info.is_none() {
        return;
    }
    commands.insert_resource(GoaledBall(0));
    commands.insert_resource(Score(0));
    commands.insert_resource(OperationAmount(0));
    commands.insert_resource(PassedTime(0));

    info!("stage setup");
    let stage_info = stage_info.unwrap().clone();
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
        block_event_writer.send(SpawnBlockEvent::from(&block));
    }
    for ball in ball_list {
        ball_event_writer.send(ball.to_spawn_event())
    }
    for goal in goal_list {
        goal_event_writer.send(goal.to_spawn_event())
    }
    commands.remove_resource::<StageInfo>();
    app_state.set(AppState::Game).unwrap();
}

pub struct LoadStagePlugin;
impl Plugin for LoadStagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Loading).with_system(stage_setup));
    }
}
