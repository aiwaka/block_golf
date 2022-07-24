use bevy::prelude::*;

use crate::{
    components::{
        game::{
            GameOverEvent, GameRule, GoaledBall, InitialBallNum, OperationAmount, PassedTime,
            ResultScore, Score,
        },
        info::RemainingTime,
        timer::CountDownTimer,
    },
    AppState,
};

use super::effects::fade::register_fade;

/// ルールによって異なる条件を満たしたらゲームオーバーイベントを送る
fn game_over_check(
    rule: Res<GameRule>,
    timer_query: Query<&CountDownTimer, With<RemainingTime>>,
    goaled_ball: Res<GoaledBall>,
    init_ball_num: Res<InitialBallNum>,
    mut game_over_event_writer: EventWriter<GameOverEvent>,
) {
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
        commands.insert_resource(ResultScore(result_score));
        info!("result score: {}", result_score);
    }
}
fn game_over(
    mut commands: Commands,
    mut game_over_event_reader: EventReader<GameOverEvent>,
    timer_ent: Query<Entity, With<RemainingTime>>,
) {
    for _ in game_over_event_reader.iter() {
        commands.entity(timer_ent.single()).despawn();
        register_fade(&mut commands, 0.1);
    }
}

pub struct GameManagePlugin;
impl Plugin for GameManagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(game_over_check.label("gameover_check")),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(
                save_result_score
                    .after("gameover_check")
                    .label("save_score"),
            ),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(game_over.after("save_score")),
        );
    }
}
