//! プレイヤーに見えるゲーム情報を表示するシステム等
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::{
    ball::{SetBallEvent, SpawnBallEvent},
    info::{NextBall, RemainingBall},
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

fn remaining_balls_info(
    mut commands: Commands,
    remaining_ball_query: Query<(&RemainingBall, Entity)>,
    changed_magazine_query: Query<&BallMagazine, Changed<BallMagazine>>,
) {
    for mag in changed_magazine_query.iter() {
        // 変化があったとき
        for (_, ent) in remaining_ball_query.iter() {
            commands.entity(ent).despawn();
        }
        for (idx, ball_type) in mag.balls.iter().enumerate() {
            let ball_shape = shapes::Circle {
                radius: 10.0,
                ..Default::default()
            };
            let show_pos = Vec2::new(-200.0 + idx as f32 * 40.0, -350.0);
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &ball_shape,
                    DrawMode::Outlined {
                        fill_mode: FillMode::color(ball_type.color()),
                        outline_mode: StrokeMode::new(Color::DARK_GRAY, 1.0),
                    },
                    Transform {
                        translation: show_pos.extend(11.0),
                        ..Default::default()
                    },
                ))
                .insert(RemainingBall);
        }
    }
}

pub struct InfoBoardPlugin;
impl Plugin for InfoBoardPlugin {
    fn build(&self, app: &mut App) {
        // app.add_startup_system(ui_setup);
        app.add_system(remaining_balls_info);
    }
}
