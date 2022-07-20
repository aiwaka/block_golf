use super::ball::BallType;
use bevy::prelude::{Component, Entity};

#[derive(Debug)]
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
    pub state: LauncherState,
    pub angle: f32,
}
