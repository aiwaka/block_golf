use super::ball::BallType;
use bevy::prelude::{Component, Entity};

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
    pub angle: f32,
}
