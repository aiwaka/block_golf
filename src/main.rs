mod components;
mod stages;
mod systems;

use bevy::{input::keyboard::keyboard_input_system, prelude::*};
use bevy_prototype_lyon::prelude::*;

use systems::{
    ball::BallPlugin, block::BlockPlugin, collision::CollisionPlugin, field::FieldPlugin,
    launcher::LauncherPlugin, setup::global_setup,
};

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        title: "Block Golf".to_string(),
        ..Default::default()
    });
    app.add_plugins(DefaultPlugins);
    app.add_system(keyboard_input_system);
    app.add_plugin(ShapePlugin);
    app.add_system(bevy::input::system::exit_on_esc_system);
    app.add_startup_system(global_setup);
    app.add_plugin(FieldPlugin);
    app.add_plugin(BallPlugin);
    app.add_plugin(BlockPlugin);
    app.add_plugin(LauncherPlugin);
    app.add_plugin(CollisionPlugin);
    app.run();
}
