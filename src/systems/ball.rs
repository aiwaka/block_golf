use crate::{
    components::{
        ball::{Ball, BallNocking, LaunchBallEvent, SpawnBallEvent},
        physics::{material::PhysicMaterial, position::Position, velocity::Velocity},
    },
    AppState,
};
use bevy::{math::vec2, prelude::*};
use bevy_prototype_lyon::prelude::*;

use super::field::{FIELD_HEIGHT, FIELD_WIDTH};

fn spawn_ball(mut commands: Commands, mut event_listener: EventReader<SpawnBallEvent>) {
    for ev in event_listener.iter() {
        let ball_shape = shapes::Circle {
            radius: ev.ball_type.radius(),
            ..Default::default()
        };
        let default_pos = Vec2::new(-FIELD_WIDTH / 2.0 + 60.0, -FIELD_HEIGHT / 2.0 + 60.0);
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(ev.ball_type.color()),
                    outline_mode: StrokeMode::new(Color::DARK_GRAY, 2.0),
                },
                Transform {
                    translation: Vec3::new(default_pos.x, default_pos.y, 11.0),
                    ..Default::default()
                },
            ))
            .insert(Ball::new(default_pos, vec2(0.0, 0.0), ev.ball_type))
            .insert(PhysicMaterial::new(
                ev.ball_type.restitution(),
                ev.ball_type.density(),
                0.0,
            ))
            .insert(Position(default_pos))
            .insert(Velocity(Vec2::new(0.0, 0.0)))
            .insert(BallNocking);
    }
}

type MarkerMovingBall = (With<Ball>, Without<BallNocking>);
type MarkerNotMovingBall = (With<Ball>, With<BallNocking>);

fn launch_ball(
    mut commands: Commands,
    mut event_listener: EventReader<LaunchBallEvent>,
    mut query: Query<(&mut Velocity, Entity), MarkerNotMovingBall>,
) {
    for ev in event_listener.iter() {
        for (mut vel, ent) in query.iter_mut() {
            vel.0 = ev.direction;
            commands.entity(ent).remove::<BallNocking>();
        }
    }
}

fn reflect_ball_pos(mut query: Query<(&Position, &mut Transform), MarkerMovingBall>) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(pos.0.x, pos.0.y, 11.0);
    }
}

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(spawn_ball));
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(reflect_ball_pos.after("move_pos")),
        );
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(launch_ball));
        // app.add_system(spawn_ball);
        // app.add_system(reflect_ball_pos.after("move_pos"));
        // app.add_system(launch_ball);
    }
}
