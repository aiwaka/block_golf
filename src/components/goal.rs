use bevy::prelude::{Component, Vec2};

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
        GoalHole {
            pos: ev.pos,
            radius: ev.radius,
            score: ev.score,
        }
    }
}

pub struct SpawnGoalEvent {
    pub pos: Vec2,
    pub radius: f32,
    pub score: u32,
}
impl SpawnGoalEvent {
    pub fn new(pos: Vec2, radius: f32, score: u32) -> Self {
        Self { pos, radius, score }
    }
}
