use bevy::prelude::*;

pub struct SpawnGoalEvent {
    pub pos: Vec2,
    pub radius: f32,
    pub score: u32,
}
