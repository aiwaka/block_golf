use super::ball::Ball;
use bevy::prelude::Component;

pub enum LauncherState {
    Waiting,
    Nocking,
}

#[derive(Component)]
pub struct Launcher {
    pub state: LauncherState,
    pub balls: Vec<Ball>,
    pub angle: f32,
}
