use crate::components::ball::{Ball, BallType};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn spawn_ball(mut commands: Commands, key_in: Res<Input<KeyCode>>) {
    if key_in.just_pressed(KeyCode::Z) {
        println!("test");
        let ball_shape = shapes::Circle {
            radius: 30.0,
            ..Default::default()
        };
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::BLUE),
                    outline_mode: StrokeMode::new(Color::DARK_GRAY, 2.0),
                },
                Transform {
                    translation: Vec3::new(0.0, 0.0, 11.0),
                    ..Default::default()
                },
            ))
            .insert(Ball::default());
    }
}

fn move_ball(commands: Commands, mut ball_query: Query<(&mut Ball, &mut Transform)>) {
    for (mut ball, mut transform) in ball_query.iter_mut() {
        let current_pos = transform.translation;
        let new_pos = Vec2::new(
            ball.direction.x + current_pos.x,
            ball.direction.y + current_pos.y,
        );
        transform.translation = Vec3::new(new_pos.x, new_pos.y, 11.0);
        ball.pos = new_pos;
    }
}

pub struct BallPlugin;
impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ball);
        app.add_system(move_ball);
    }
}
