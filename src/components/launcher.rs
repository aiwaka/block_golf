use bevy::prelude::Component;

use super::ball::Ball;

#[derive(Component)]
pub struct Launcher {
    pub balls: Vec<Ball>,
}
