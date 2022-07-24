use bevy::prelude::*;

use crate::{
    components::{
        game::{GameOverEvent, GameRule, GoaledBall, InitialBallNum},
        info::RemainingTime,
        timer::CountDownTimer,
    },
    AppState,
};

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

pub struct GameManagePlugin;
impl Plugin for GameManagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(game_over_check));
    }
}
