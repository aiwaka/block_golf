use super::ball::BallType;
use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub enum LauncherState {
    Waiting,
    Nocking,
}

#[derive(Component, Debug)]
pub struct BallMagazine {
    pub balls: Vec<(BallType, Entity)>,
}

#[derive(Component, Debug)]
pub struct Launcher {
    pub angle: f32,        // 角度（状態変数）
    pub rotate_speed: f32, // 回転角速度
    pub min_angle: f32,    // 角度下限
    pub max_angle: f32,    // 角度上限
}

pub struct SpawnLauncherEvent {
    pub pos: Vec2,
    pub rotate_speed: f32,
    pub min_angle: f32,
    pub max_angle: f32,
}
