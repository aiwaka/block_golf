use bevy::prelude::*;

use crate::AppState;

use crate::components::game::{GoaledBall, OperationAmount, PassedTime, Score};
use crate::components::timer::CountDownTimer;

#[derive(Component)]
pub struct Loading;

pub fn stage_setup(mut commands: Commands, loading: Query<&Loading>) {
    // ローディング中なら実行しない
    if !loading.is_empty() {
        return;
    }
    // info!("stage setup");
    commands.insert_resource(Loading);

    commands.insert_resource(GoaledBall(0));
    commands.insert_resource(Score(0));
    commands.insert_resource(OperationAmount(0));
    commands.insert_resource(PassedTime(0));

    commands
        .spawn()
        .insert(Loading)
        .insert(CountDownTimer::new(20));
}

/// 一瞬ロード待ちを入れる（リソースの追加消去を確実にするため）
fn wait_for_a_moment(
    timer: Query<&CountDownTimer, With<Loading>>,
    mut app_state: ResMut<State<AppState>>,
) {
    if let Ok(timer) = timer.get_single() {
        if timer.is_finished() {
            app_state.set(AppState::Game).unwrap();
        }
    }
}

pub struct LoadStagePlugin;
impl Plugin for LoadStagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Loading).with_system(stage_setup));
        app.add_system_set(
            SystemSet::on_update(AppState::Loading)
                .with_system(wait_for_a_moment.after("count_down_update")),
        );
    }
}
