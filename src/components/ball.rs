use bevy::prelude::*;

pub enum BallType {
    Normal,
}

#[derive(Component)]
pub struct Ball {
    pub direction: Vec2, // 絶対値をspeedとする
    pub ball_type: BallType,
}
impl Default for Ball {
    fn default() -> Self {
        Ball {
            direction: Vec2::new(4.0, 0.0),
            ball_type: BallType::Normal,
        }
    }
}
