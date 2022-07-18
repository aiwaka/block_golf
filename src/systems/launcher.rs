use super::field::{FIELD_HEIGHT, FIELD_WIDTH};
use crate::components::{
    ball::{BallType, LaunchBallEvent, SpawnBallEvent},
    launcher::{Launcher, LauncherState},
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
        .insert(Launcher {
            state: LauncherState::Waiting,
            balls: vec![],
            angle: 0.0,
        });
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

fn launch_ball(
    key_in: Res<Input<KeyCode>>,
    mut spawn_ball_event_writer: EventWriter<SpawnBallEvent>,
    mut launch_ball_event_writer: EventWriter<LaunchBallEvent>,
    mut query: Query<&mut Launcher>,
) {
    if key_in.just_pressed(KeyCode::Z) {
        for mut launcher in query.iter_mut() {
            match launcher.state {
                LauncherState::Waiting => {
                    launcher.state = LauncherState::Nocking;
                    spawn_ball_event_writer.send(SpawnBallEvent {
                        ball_type: BallType::Normal,
                    });
                }
                LauncherState::Nocking => {
                    launcher.state = LauncherState::Waiting;
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
        app.add_startup_system(spawn_launcher.after("stage_setup"));
        app.add_system(rotate_launcher);
        app.add_system(launch_ball);
    }
}
