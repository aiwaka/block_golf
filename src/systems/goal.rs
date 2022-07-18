use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::components::{
    goal::{GoalHole, SpawnGoalEvent},
    physics::{position::Position, velocity::Velocity},
};

fn spawn_goal(mut commands: Commands, mut event_listener: EventReader<SpawnGoalEvent>) {
    for ev in event_listener.iter() {
        let goal_shape = shapes::Circle {
            radius: ev.radius,
            ..Default::default()
        };
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &goal_shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::GRAY),
                    outline_mode: StrokeMode::new(Color::DARK_GRAY, 2.0),
                },
                Transform {
                    translation: ev.pos.extend(11.0),
                    ..Default::default()
                },
            ))
            .insert(GoalHole::from(ev))
            .insert(Position(ev.pos))
            .insert(Velocity(Vec2::new(0.0, 0.0)));
    }
}

pub struct GoalPlugin;
impl Plugin for GoalPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_goal.after("stage_setup"));
    }
}
