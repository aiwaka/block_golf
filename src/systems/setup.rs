use bevy::prelude::*;

pub fn global_setup(mut commands: Commands) {
    // カメラのセット
    commands.spawn_bundle(Camera2dBundle::default());
}
