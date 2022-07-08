use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Polygon};

use crate::components::launcher::Launcher;

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
                translation: Vec3::new(-100.0, -100.0, 15.0),
                ..Default::default()
            },
        ))
        .insert(Launcher {
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
        if launcher.angle > FRAC_PI_2 {
            launcher.angle = FRAC_PI_2;
        } else if launcher.angle < 0.0 {
            launcher.angle = 0.0;
        }
        trans.rotation = Quat::from_rotation_z(launcher.angle);
    }
}

pub struct LauncherPlugin;
impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_launcher);
        app.add_system(rotate_launcher);
    }
}
