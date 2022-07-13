use crate::components::ball::{Ball, BallNocking, LaunchBallEvent, SpawnBallEvent};
use bevy::{math::vec2, prelude::*};
use bevy_prototype_lyon::prelude::*;

use super::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub const BALL_RADIUS: f32 = 30.0;

fn spawn_ball(mut commands: Commands, mut event_listener: EventReader<SpawnBallEvent>) {
    for ev in event_listener.iter() {
        let ball_shape = shapes::Circle {
            radius: BALL_RADIUS,
            ..Default::default()
        };
        let default_pos = Vec2::new(-FIELD_WIDTH / 2.0 + 60.0, -FIELD_HEIGHT / 2.0 + 60.0);
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::BLUE),
                    outline_mode: StrokeMode::new(Color::DARK_GRAY, 2.0),
                },
                Transform {
                    translation: Vec3::new(default_pos.x, default_pos.y, 11.0),
                    ..Default::default()
                },
            ))
            .insert(Ball::new(default_pos, vec2(0.0, 0.0), ev.ball_type))
            .insert(BallNocking);
    }
}

fn launch_ball(
    mut commands: Commands,
    mut event_listener: EventReader<LaunchBallEvent>,
    mut query: Query<(&mut Ball, Entity), With<BallNocking>>,
) {
    for ev in event_listener.iter() {
        for (mut ball, ball_ent) in query.iter_mut() {
            ball.direction = ev.direction;
            commands.entity(ball_ent).remove::<BallNocking>();
        }
    }
}

fn move_ball(mut ball_query: Query<(&mut Ball, &mut Transform), Without<BallNocking>>) {
    for (mut ball, mut transform) in ball_query.iter_mut() {
        let direction = ball.direction;
        ball.pos += direction;
        transform.translation = Vec3::new(ball.pos.x, ball.pos.y, 11.0);
    }
}

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBallEvent>();
        app.add_event::<LaunchBallEvent>();
        app.add_system(spawn_ball);
        app.add_system(move_ball);
        app.add_system(launch_ball);
    }
}
