use bevy::prelude::*;

pub fn global_setup(mut commands: Commands) {
    // カメラのセット
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}
