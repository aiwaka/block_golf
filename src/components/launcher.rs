use super::ball::BallType;
use bevy::prelude::Component;

#[derive(Debug)]
pub enum LauncherState {
    Waiting,
    Nocking,
}

#[derive(Component, Debug)]
pub struct Launcher {
    pub state: LauncherState,
    pub balls: Vec<BallType>,
    pub angle: f32,
}
