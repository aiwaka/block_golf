mod components;
mod stages;
mod systems;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use components::{
    ball::{LaunchBallEvent, SetBallEvent, SpawnBallEvent},
    block::SpawnBlockEvent,
    game::GameOverEvent,
    goal::SpawnGoalEvent,
};
use systems::{
    ball::BallPlugin,
    block::BlockPlugin,
    collision::CollisionPlugin,
    effects::EffectPlugin,
    field::FieldPlugin,
    game::GameManagePlugin,
    goal::GoalPlugin,
    info_board::InfoBoardPlugin,
    launcher::LauncherPlugin,
    main_menu::MainMenuPlugin,
    physics::motion_dynamics::MotionDynamicsPlugin,
    setup::{global_setup, stage_setup},
    timer::TimersPlugin,
};

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Menu,
    Loading,
    Game,
    Result,
}

fn add_events(app: &mut App) {
    app.add_event::<SpawnBallEvent>();
    app.add_event::<LaunchBallEvent>();
    app.add_event::<SpawnBlockEvent>();
    app.add_event::<SpawnGoalEvent>();
    app.add_event::<SetBallEvent>();
    app.add_event::<GameOverEvent>();
}

fn main() {
    let window = WindowDescriptor {
        title: "Block Golf".to_string(),
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        resizable: false,
        ..Default::default()
    };
    let mut app = App::new();
    app.insert_resource(window);
    app.add_plugins(DefaultPlugins);
    app.add_system(bevy::input::system::exit_on_esc_system);
    app.add_plugin(ShapePlugin);
    add_events(&mut app);
    app.add_state(AppState::Menu);

    app.add_startup_system(global_setup.label("global_setup"));
    app.add_plugin(MainMenuPlugin);
    app.add_plugin(EffectPlugin);
    app.add_system_set(
        SystemSet::on_enter(AppState::Game).with_system(stage_setup.label("stage_setup")),
    );
    app.add_plugin(FieldPlugin);
    app.add_plugin(GoalPlugin);
    app.add_plugin(BallPlugin);
    app.add_plugin(BlockPlugin);
    app.add_plugin(CollisionPlugin);
    app.add_plugin(LauncherPlugin);
    app.add_plugin(MotionDynamicsPlugin);
    app.add_plugin(InfoBoardPlugin);
    app.add_plugin(TimersPlugin);
    app.add_plugin(GameManagePlugin);
    app.run();
}
