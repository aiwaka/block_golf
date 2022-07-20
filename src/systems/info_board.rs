//! プレイヤーに見えるゲーム情報を表示するシステム等
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::{
    ball::{SetBallEvent, SpawnBallEvent},
    info::{ConsumingBall, MagazineUpdating, RemainingBall},
    launcher::{BallMagazine, Launcher},
};

// fn ui_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn_bundle(TextBundle {
//         style: Style {
//             align_self: AlignSelf::FlexEnd,
//             position_type: PositionType::Absolute,
//             position: Rect {
//                 bottom: Val::Px(5.0),
//                 right: Val::Px(15.0),
//                 ..default()
//             },
//             ..default()
//         },
//         // Use the `Text::with_section` constructor
//         text: Text::with_section(
//             // Accepts a `String` or any type that converts into a `String`, such as `&str`
//             "hello\nbevy!",
//             TextStyle {
//                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
//                 font_size: 100.0,
//                 color: Color::BLACK,
//             },
//             // Note: You can use `Default::default()` in place of the `TextAlignment`
//             TextAlignment {
//                 horizontal: HorizontalAlign::Center,
//                 ..default()
//             },
//         ),
//         ..default()
//     });
// }

/// ボール出現時に箱に更新中マーカーを付与する
fn insert_updateing_marker(
    mut commands: Commands,
    magazine_query: Query<(&BallMagazine, Entity)>,
    mut spawn_ball_event_reader: EventReader<SpawnBallEvent>,
) {
    for _ in spawn_ball_event_reader.iter() {
        let (mag, ent) = magazine_query.single();
        commands.entity(ent).insert(MagazineUpdating);
        // 1つ目に消費中マーカーをつける
        commands.entity(mag.balls[0].1).insert(ConsumingBall);
    }
}

fn update_remaining_balls_info(
    mut commands: Commands,
    mut remaining_ball_query: Query<
        (&mut Transform, Option<&ConsumingBall>, Entity),
        With<RemainingBall>,
    >,
    mut magazine_query: Query<(&mut BallMagazine, Entity), With<MagazineUpdating>>,
) {
    if magazine_query.iter().next().is_none() {
        return;
    }
    if let Some((mut trans, _, ent)) = remaining_ball_query.iter_mut().find(|q| q.1.is_some()) {
        // consumingballが存在していたらその処理だけ行う
        trans.scale *= 0.8;
        if trans.scale.x < 0.01 {
            commands.entity(ent).despawn();
        }
    } else {
        let (mut mag, ent) = magazine_query.single_mut();
        // TODO: VecDequeを使うと計算量を減らせる
        mag.balls.remove(0);
        commands.entity(ent).remove::<MagazineUpdating>();
    }
}

pub struct InfoBoardPlugin;
impl Plugin for InfoBoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(insert_updateing_marker);
        app.add_system(update_remaining_balls_info);
    }
}
