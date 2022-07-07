use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

enum BallType {
    Normal,
}

#[derive(Component)]
pub struct Ball {
    pos: Vec2,
    direction: Vec2, // 絶対値をspeedとする
    _type: BallType,
}
impl Default for Ball {
    fn default() -> Self {
        Ball {
            pos: Vec2::ZERO,
            direction: Vec2::new(4.0, 0.0),
            _type: BallType::Normal,
        }
    }
}

fn spawn_ball(mut commands: Commands, key_in: Res<Input<KeyCode>>) {
    if key_in.just_pressed(KeyCode::Z) {
        let ball_shape = shapes::Circle {
            radius: 30.0,
            ..Default::default()
        };
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &ball_shape,
                DrawMode::Fill(FillMode::color(Color::BLUE)),
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
