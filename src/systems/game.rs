use bevy::prelude::*;

use crate::{
    components::{
        ball::SetBallEvent,
        block::SpawnBlockEvent,
        game::{
            GameOverEvent, GameRule, GoaledBall, InitialBallNum, NowGameOver, OperationAmount,
            PassedTime, ResultInfoStorage, Score,
        },
        goal::SpawnGoalEvent,
        info::{RemainingTime, WaitForResultDisplay},
        launcher::SpawnLauncherEvent,
        timer::CountDownTimer,
    },
    stages::structs::StageInfo,
    AppState,
};

use super::effects::fade::register_fade;

struct ResidentEntities(Vec<Entity>);
fn init_game(mut commands: Commands, entities: Query<Entity>) {
    // 最初に存在しているentityをすべて保存しておく.
    commands.insert_resource(ResidentEntities(entities.iter().collect::<Vec<Entity>>()));
}

/// ステージ情報から様々なものを出現させる
fn spawn_stage_entities(
    mut commands: Commands,
    stage_info: Option<Res<StageInfo>>,
    mut launcher_event_writer: EventWriter<SpawnLauncherEvent>,
    mut block_event_writer: EventWriter<SpawnBlockEvent>,
    mut goal_event_writer: EventWriter<SpawnGoalEvent>,
    mut ball_event_writer: EventWriter<SetBallEvent>,
) {
    // info!("spawn stage entities");
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
}

/// スコアに関わる値を更新する
fn update_score_resources(
    key_in: Res<Input<KeyCode>>,
    mut operation_amount: ResMut<OperationAmount>,
    mut passed_time: ResMut<PassedTime>,
) {
    if key_in.any_pressed([KeyCode::Left, KeyCode::Right]) {
        operation_amount.0 += 1;
    }
    passed_time.0 += 1;
}

/// ルールによって異なる条件を満たしたらゲームオーバーイベントを送る
fn game_over_check(
    rule: Res<GameRule>,
    timer_query: Query<&CountDownTimer, With<RemainingTime>>,
    goaled_ball: Res<GoaledBall>,
    init_ball_num: Res<InitialBallNum>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
    is_gameover: Option<Res<NowGameOver>>,
) {
    if is_gameover.is_none() {
        if let Ok(timer) = timer_query.get_single() {
            if timer.is_finished()
                || match *rule {
                    GameRule::BallScore => init_ball_num.0 == goaled_ball.0,
                    GameRule::LittleOperation => goaled_ball.0 != 0,
                    GameRule::TimeAttack => goaled_ball.0 != 0,
                }
            {
                info!("send game over event");
                game_over_event_writer.send(GameOverEvent);
            }
        }
    }
}

/// ゲームオーバーイベントを受け取ったらスコアを計算し保存する.
fn save_result_score(
    mut commands: Commands,
    rule: Res<GameRule>,
    score: Res<Score>,
    operation_amount: Res<OperationAmount>,
    passed_time: Res<PassedTime>,
    mut game_over_event_reader: EventReader<GameOverEvent>,
) {
    for _ in game_over_event_reader.iter() {
        let result_score = match *rule {
            GameRule::BallScore => score.0,
            GameRule::LittleOperation => operation_amount.0,
            GameRule::TimeAttack => passed_time.0,
        };
        commands.insert_resource(ResultInfoStorage {
            score: result_score,
        });
        info!("result score: {}", result_score);
    }
}
/// 実際にゲームオーバーのフラグ立て等の処理を行う（チェックと同時にやってもいいがクエリが煩雑なので分けた）
fn game_over(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOverEvent>,
    mut timer_query: Query<&mut CountDownTimer, With<RemainingTime>>,
) {
    for _ in game_over_event_reader.iter() {
        if let Ok(mut timer) = timer_query.get_single_mut() {
            timer.stop();
        }
        commands.insert_resource(NowGameOver);
        commands
            .spawn()
            .insert(WaitForResultDisplay)
            .insert(CountDownTimer::new(30));
        register_fade(&mut commands, 0.01, Color::rgba(0.0, 0.0, 0.0, 0.8));
    }
}

fn return_to_title(
    is_gameover: Option<Res<NowGameOver>>,
    timer_query: Query<&WaitForResultDisplay>,
    key_in: Res<Input<KeyCode>>,
    mut app_state: ResMut<State<AppState>>,
) {
    // ゲームオーバーでかつ待ちタイマーが存在しなければメニューに戻る受付をする
    if is_gameover.is_none() || !timer_query.is_empty() {
        return;
    }
    if key_in.just_pressed(KeyCode::Z) {
        app_state.set(AppState::BackToMenu).unwrap();
    }
}

/// ステージを最初からやり直す
fn retry(
    mut commands: Commands,
    key_in: Res<Input<KeyCode>>,
    timer_query: Query<Entity, (With<CountDownTimer>, With<RemainingTime>)>,
    mut app_state: ResMut<State<AppState>>,
) {
    if key_in.just_pressed(KeyCode::R) {
        // タイマー削除する
        for ent in timer_query.iter() {
            commands.entity(ent).despawn();
        }
        app_state.set(AppState::Loading).unwrap();
    }
}

/// Menu状態の初期からあったものを除いたすべてのEntityを削除する
fn deconstruct_objects(
    mut commands: Commands,
    timer_query: Query<Entity, (With<CountDownTimer>, With<RemainingTime>)>,
    entities: Query<Entity>,
    resident_entities: Res<ResidentEntities>,
) {
    for ent in entities.iter() {
        if !resident_entities.0.contains(&ent) {
            commands.entity(ent).despawn();
        }
    }
    // タイマーも残っていたら削除する
    for ent in timer_query.iter() {
        commands.entity(ent).despawn();
    }
    commands.remove_resource::<NowGameOver>();
    commands.remove_resource::<ResultInfoStorage>();
}

pub struct GameManagePlugin;
impl Plugin for GameManagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(init_game));
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(spawn_stage_entities)
                .label("spawn_stage_entities"),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(update_score_resources),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(
                game_over_check
                    .label("gameover_check")
                    .after("count_down_update"),
            ),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(
                save_result_score
                    .after("gameover_check")
                    .label("save_score"),
            ),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(game_over.after("save_score").after("count_down_update")),
        );
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(return_to_title));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(retry));
        app.add_system_set(
            SystemSet::on_exit(AppState::Game)
                .with_system(deconstruct_objects.label("deconstruct")),
        );
    }
}
