use bevy::prelude::*;

#[derive(Component)]
pub struct FadeEffect {
    pub alpha: f32,
    pub delta: f32,
    pub finished: bool,
}
