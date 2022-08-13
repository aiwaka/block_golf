use bevy::prelude::Vec2;

pub struct SpawnLauncherEvent {
    pub pos: Vec2,
    pub default_angle: f32,
    pub rotate_speed: f32,
    pub min_angle: f32,
    pub max_angle: f32,
}
