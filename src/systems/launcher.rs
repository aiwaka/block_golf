use super::field::{FIELD_HEIGHT, FIELD_WIDTH};
use crate::{
    components::{
        ball::{BallType, LaunchBallEvent, SetBallEvent, SpawnBallEvent},
        info::RemainingBall,
        launcher::{BallMagazine, Launcher, LauncherState},
    },
    AppState,
};
use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Polygon};
use std::f32::consts::FRAC_PI_2;

fn construct_launcher_shape() -> Polygon {
    const LAUNCHER_WIDTH: f32 = 50.0;
    shapes::Polygon {
        points: vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, -LAUNCHER_WIDTH),
            Vec2::new(30.0, -LAUNCHER_WIDTH),
            Vec2::new(30.0, -LAUNCHER_WIDTH * 0.8),
            Vec2::new(10.0, -LAUNCHER_WIDTH * 0.8),
            Vec2::new(10.0, LAUNCHER_WIDTH * 0.8),
            Vec2::new(30.0, LAUNCHER_WIDTH * 0.8),
            Vec2::new(30.0, LAUNCHER_WIDTH),
            Vec2::new(0.0, LAUNCHER_WIDTH),
        ],
        closed: true,
    }
}

fn spawn_ball_magazine(mut commands: Commands, mut ball_event_reader: EventReader<SetBallEvent>) {
    let mut balls = Vec::<(BallType, Entity)>::new();
    for (idx, ev) in ball_event_reader.iter().enumerate() {
        let ball_shape = shapes::Circle {
            radius: 10.0,
            ..Default::default()
        };
        let show_pos = Vec2::new(-200.0 + idx as f32 * 40.0, -350.0);
        let ent = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(ev.ball_type.color()),
                    outline_mode: StrokeMode::new(Color::DARK_GRAY, 1.0),
                },
                Transform {
                    translation: show_pos.extend(11.0),
                    ..Default::default()
                },
            ))
            .insert(RemainingBall)
            .id();
        balls.push((ev.ball_type, ent));
    }
    commands.spawn().insert(BallMagazine { balls });
}

fn spawn_launcher(mut commands: Commands) {
    let shape = construct_launcher_shape();
    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Outlined {
                fill_mode: FillMode::color(Color::BLUE),
                outline_mode: StrokeMode::new(Color::DARK_GRAY, 2.0),
            },
            Transform {
                translation: Vec3::new(-FIELD_WIDTH / 2.0 + 30.0, -FIELD_HEIGHT / 2.0 + 30.0, 15.0),
                ..Default::default()
            },
        ))
        .insert(Launcher { angle: 0.0 })
        .insert(LauncherState::Waiting);
}

fn rotate_launcher(key_in: Res<Input<KeyCode>>, mut query: Query<(&mut Transform, &mut Launcher)>) {
    const LAUNCHER_ROTATE_ANGLE: f32 = 0.02;
    for (mut trans, mut launcher) in query.iter_mut() {
        if key_in.pressed(KeyCode::Right) {
            launcher.angle -= LAUNCHER_ROTATE_ANGLE;
        } else if key_in.pressed(KeyCode::Left) {
            launcher.angle += LAUNCHER_ROTATE_ANGLE;
        }
        if launcher.angle > FRAC_PI_2 * 1.2 {
            launcher.angle = FRAC_PI_2 * 1.2;
        } else if launcher.angle < FRAC_PI_2 * -0.2 {
            launcher.angle = FRAC_PI_2 * -0.2;
        }
        trans.rotation = Quat::from_rotation_z(launcher.angle);
    }
}

fn nock_ball(
    mut commands: Commands,
    key_in: Res<Input<KeyCode>>,
    mut spawn_ball_event_writer: EventWriter<SpawnBallEvent>,
    query: Query<(&Launcher, &LauncherState, Entity)>,
    magazine_query: Query<&BallMagazine>,
) {
    if key_in.just_pressed(KeyCode::Z) {
        for (_, state, ent) in query.iter() {
            if let LauncherState::Waiting = *state {
                // 待機状態ならボールを一つ読み取ってボール出現イベントを送信
                let magazine = magazine_query.single();
                let ball_type = if let Some((ball_type, _)) = magazine.balls.get(0) {
                    *ball_type
                } else {
                    // 残りボールが無い状態. 効果音とか鳴らすようにするとよさそう
                    continue;
                };
                commands
                    .entity(ent)
                    .remove::<LauncherState>()
                    .insert(LauncherState::Nocking);
                spawn_ball_event_writer.send(SpawnBallEvent { ball_type });
            }
        }
    }
}

fn launch_ball(
    mut commands: Commands,
    key_in: Res<Input<KeyCode>>,
    mut launch_ball_event_writer: EventWriter<LaunchBallEvent>,
    query: Query<(&Launcher, &LauncherState, Entity)>,
) {
    if key_in.just_pressed(KeyCode::Z) {
        for (launcher, state, ent) in query.iter() {
            match *state {
                LauncherState::Waiting => {}
                LauncherState::Nocking => {
                    commands
                        .entity(ent)
                        .remove::<LauncherState>()
                        .insert(LauncherState::Waiting);
                    launch_ball_event_writer.send(LaunchBallEvent {
                        direction: 5.0 * Vec2::new(launcher.angle.cos(), launcher.angle.sin()),
                    });
                }
            }
        }
    }
}

pub struct LauncherPlugin;
impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game).with_system(spawn_launcher.after("stage_setup")),
        );
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(spawn_ball_magazine.after("stage_setup")),
        );
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(rotate_launcher));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(nock_ball));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(launch_ball));
        // app.add_startup_system(spawn_launcher.after("stage_setup"));
        // app.add_startup_system(spawn_ball_magazine.after("stage_setup"));
        // app.add_system(rotate_launcher);
        // app.add_system(nock_ball);
        // app.add_system(launch_ball);
    }
}
