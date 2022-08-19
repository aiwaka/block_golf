use bevy::prelude::{Component, Vec2};

use crate::events::goal::SpawnGoalEvent;

#[derive(Component)]
pub struct GoalHole {
    pub pos: Vec2,
    pub radius: f32,
    pub score: u32,
}
impl GoalHole {
    pub fn new(pos: Vec2, radius: f32, score: u32) -> Self {
        Self { pos, radius, score }
    }
}
impl From<&SpawnGoalEvent> for GoalHole {
    fn from(ev: &SpawnGoalEvent) -> Self {
        GoalHole::new(ev.pos, ev.radius, ev.score)
    }
}
