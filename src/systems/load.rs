use bevy::prelude::*;

use crate::AppState;

use crate::components::game::{GoaledBall, OperationAmount, PassedTime, Score};

pub struct Loading;

pub fn stage_setup(mut commands: Commands, loading: Option<Res<Loading>>) {
    // ローディング中でなく
    if loading.is_some() {
        return;
    }
    commands.insert_resource(Loading);

    commands.insert_resource(GoaledBall(0));
    commands.insert_resource(Score(0));
    commands.insert_resource(OperationAmount(0));
    commands.insert_resource(PassedTime(0));
}

/// 20フレーム待つためのローカルリソース
struct LocalWaitCount(u32);
impl Default for LocalWaitCount {
    fn default() -> Self {
        LocalWaitCount(20)
    }
}
/// 一瞬ロード待ちを入れる（リソースの追加消去を確実にするため）
fn wait_for_a_moment(
    mut commands: Commands,
    mut wait_count: Local<LocalWaitCount>,
    mut app_state: ResMut<State<AppState>>,
) {
    if wait_count.0 > 0 {
        wait_count.0 -= 1;
        info!("{}", wait_count.0);
    } else {
        commands.remove_resource::<Loading>();
        app_state.set(AppState::Game).unwrap();
    }
}

pub struct LoadStagePlugin;
impl Plugin for LoadStagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Loading).with_system(stage_setup));
        app.add_system_set(SystemSet::on_update(AppState::Loading).with_system(wait_for_a_moment));
    }
}
