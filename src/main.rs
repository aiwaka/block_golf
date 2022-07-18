mod components;
mod stages;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use components::{
    ball::{LaunchBallEvent, SpawnBallEvent},
    block::SpawnBlockEvent,
    goal::SpawnGoalEvent,
};
use systems::{
    ball::BallPlugin,
    block::BlockPlugin,
    collision::CollisionPlugin,
    field::FieldPlugin,
    goal::GoalPlugin,
    launcher::LauncherPlugin,
    physics::motion_dynamics::MotionDynamicsPlugin,
    setup::{global_setup, stage_setup},
};

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

fn add_events(app: &mut App) {
    app.add_event::<SpawnBallEvent>();
    app.add_event::<LaunchBallEvent>();
    app.add_event::<SpawnBlockEvent>();
    app.add_event::<SpawnGoalEvent>();
}

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        title: "Block Golf".to_string(),
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_system(bevy::input::system::exit_on_esc_system);
    app.add_plugin(ShapePlugin);
    add_events(&mut app);
    app.add_startup_system(global_setup.label("global_setup"));
    app.add_startup_system(stage_setup.label("stage_setup").after("global_setup"));
    app.add_plugin(FieldPlugin);
    app.add_plugin(GoalPlugin);
    app.add_plugin(BallPlugin);
    app.add_plugin(BlockPlugin);
    app.add_plugin(CollisionPlugin);
    app.add_plugin(LauncherPlugin);
    app.add_plugin(MotionDynamicsPlugin);
    app.run();
}
