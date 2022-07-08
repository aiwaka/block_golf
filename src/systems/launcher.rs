use bevy::prelude::*;
use bevy_prototype_lyon::{prelude::*, shapes::Polygon};

fn construct_launcher_shape() -> Polygon {
    const LAUNCHER_WIDTH: f32 = 50.0;
    shapes::Polygon {
        points: vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(LAUNCHER_WIDTH, 0.0),
            Vec2::new(LAUNCHER_WIDTH, 30.0),
            Vec2::new(40.0, 30.0),
            Vec2::new(40.0, 10.0),
            Vec2::new(-40.0, 10.0),
            Vec2::new(-40.0, 30.0),
            Vec2::new(-LAUNCHER_WIDTH, 30.0),
            Vec2::new(-LAUNCHER_WIDTH, 0.0),
        ],
        closed: true,
    }
}

fn spawn_launcher(mut commands: Commands) {
    let shape = construct_launcher_shape();
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::BLUE),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, 2.0),
        },
        Transform {
            translation: Vec3::new(-100.0, -100.0, 15.0),
            ..Default::default()
        },
    ));
}

pub struct LauncherPlugin;
impl Plugin for LauncherPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_launcher);
    }
}
