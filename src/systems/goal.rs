use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    components::{
        ball::GoalinBall,
        game::{GoaledBall, Score},
        goal::GoalHole,
        physics::{position::Position, velocity::Velocity},
    },
    events::goal::SpawnGoalEvent,
    AppState,
};

fn spawn_goal(mut commands: Commands, mut event_listener: EventReader<SpawnGoalEvent>) {
    for ev in event_listener.iter() {
        let goal_shape = shapes::Circle {
            radius: ev.radius,
            ..Default::default()
        };
        let outline_color = match ev.score {
            1 => Color::BLUE,
            2 => Color::YELLOW_GREEN,
            3 => Color::RED,
            _ => Color::DARK_GRAY,
        };
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &goal_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(outline_color, 8.0),
                },
                Transform {
                    translation: ev.pos.extend(8.0),
                    ..Default::default()
                },
            ))
            .insert(GoalHole::from(ev))
            .insert(Position(ev.pos))
            .insert(Velocity(Vec2::new(0.0, 0.0)));
    }
}

fn execute_goaled_in_ball(
    mut commands: Commands,
    mut ball_query: Query<(&mut Transform, &GoalinBall, Entity)>,
    mut goaled_ball: ResMut<GoaledBall>,
    mut score: ResMut<Score>,
) {
    for (mut trans, ball, ent) in ball_query.iter_mut() {
        trans.scale *= 0.9;
        if trans.scale.x < 0.05 {
            commands.entity(ent).despawn();
            goaled_ball.0 += 1;
            info!("goaled ball: {}", goaled_ball.0);
            score.0 += ball.0;
        }
    }
}

pub struct GoalPlugin;
impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(spawn_goal)
                .after("spawn_stage_entities"),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(execute_goaled_in_ball),
        );
    }
}
