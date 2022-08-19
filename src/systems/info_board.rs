//! プレイヤーに見えるゲーム情報を表示するシステム等
use bevy::prelude::*;

use crate::{
    components::{
        game::{NowGameOver, ResultInfoStorage},
        info::{ConsumingBall, RemainingBall, RemainingTime, ResultText, WaitForResultDisplay},
        launcher::BallMagazine,
        timer::CountDownTimer,
    },
    events::ball::SpawnBallEvent,
    AppState,
};

/// フレーム数を秒数の文字列に変換
fn frame_to_second(frame: u32) -> String {
    format!("{:>02}", frame / 60)
}

fn init_note_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(20.0),
                ..default()
            },
            ..default()
        },
        text: Text::from_section(
            "R: retry\nB: back to title",
            TextStyle {
                font: asset_server.load("fonts/ume-tgs5.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    });
}

fn init_timer_display(
    mut commands: Commands,
    timer_query: Query<(&CountDownTimer, Entity), Added<RemainingTime>>,
    asset_server: Res<AssetServer>,
) {
    // Addedでクエリを見て一度だけ実行されるようにする
    if let Ok((timer, timer_ent)) = timer_query.get_single() {
        commands.entity(timer_ent).insert_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(20.0),
                    ..default()
                },
                ..default()
            },
            text: Text::from_section(
                frame_to_second(timer.count()),
                TextStyle {
                    font: asset_server.load("fonts/ume-tgs5.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        });
    }
}

/// タイマー表示
fn show_remaining_time(mut timer_text: Query<(&mut Text, &CountDownTimer), With<RemainingTime>>) {
    if let Ok((mut text, timer)) = timer_text.get_single_mut() {
        text.sections[0].value = frame_to_second(timer.count());
    }
}

/// ボール出現時に箱の先頭のボールに更新中マーカーを付与し, 箱から取り出す
fn pop_ball_from_magazine(
    mut commands: Commands,
    mut magazine_query: Query<&mut BallMagazine>,
    mut spawn_ball_event_reader: EventReader<SpawnBallEvent>,
) {
    for _ in spawn_ball_event_reader.iter() {
        let mut mag = magazine_query.single_mut();
        // TODO: VecDequeを使うと計算量を減らせる
        let (_, top_ball_ent) = mag.balls.remove(0);
        // 1つ目に消費中マーカーをつける
        commands.entity(top_ball_ent).insert(ConsumingBall);
    }
}

fn update_remaining_balls_info(
    mut commands: Commands,
    mut remaining_ball_query: Query<
        (&mut Transform, Option<&ConsumingBall>, Entity),
        With<RemainingBall>,
    >,
) {
    if let Some((mut trans, _, ent)) = remaining_ball_query.iter_mut().find(|q| q.1.is_some()) {
        // consumingballが存在していたらその処理だけ行う
        trans.scale *= 0.8;
        if trans.scale.x < 0.01 {
            commands.entity(ent).despawn();
        }
    }
}

fn spawn_result_score(
    mut commands: Commands,
    wait_timer: Query<&CountDownTimer, With<WaitForResultDisplay>>,
    is_gameover: Option<Res<NowGameOver>>,
    result_info: Option<Res<ResultInfoStorage>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(wait_timer) = wait_timer.get_single() {
        if is_gameover.is_some() && wait_timer.is_finished() {
            if let Some(result_info) = result_info {
                let display_contents = result_info.to_vector();
                // ゲームオーバー中にタイマーが終了したら演出を開始させる
                for (title, value) in display_contents.into_iter() {
                    commands
                        .spawn_bundle(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    top: Val::Px(140.0),
                                    left: Val::Px(40.0),
                                    ..default()
                                },
                                ..default()
                            },
                            text: Text::from_section(
                                format!("{}: {}", title, value),
                                TextStyle {
                                    font: asset_server.load("fonts/ume-tgs5.ttf"),
                                    font_size: 40.0,
                                    color: Color::WHITE,
                                },
                            ),
                            ..default()
                        })
                        .insert(ResultText);
                }
            };
            commands
                .spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: UiRect {
                            bottom: Val::Px(20.0),
                            right: Val::Px(40.0),
                            ..default()
                        },
                        ..default()
                    },
                    text: Text::from_section(
                        "press Z to back.",
                        TextStyle {
                            font: asset_server.load("fonts/ume-tgs5.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                })
                .insert(ResultText);
        }
    }
}

pub struct InfoBoardPlugin;
impl Plugin for InfoBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(init_note_text));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(init_timer_display));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(show_remaining_time));
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(pop_ball_from_magazine),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(update_remaining_balls_info),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(spawn_result_score.after("count_down_update")),
        );
    }
}
