use bevy::prelude::*;

use super::field_blocks::field_block;
use super::StageInfo;
use crate::components::{
    ball::{BallType, SetBall, SetBallEvent},
    block::{BlockType, RotateStrategy, SlideStrategy, SpawnBlockEvent},
    goal::SpawnGoalEvent,
};

pub fn debug_stage() -> StageInfo {
    let block_list = vec![SpawnBlockEvent::from_type(
        {
            BlockType::Rect {
                pos: Vec2::new(0.0, 0.0),
                extents: Vec2::new(50.0, 600.0),
                rect_origin: Vec2::ZERO,
                rotate_strategy: RotateStrategy::NoRotate,
                slide_strategy: SlideStrategy::NoSlide,
                weight: 1.0,
                friction: 0.0,
                restitution: 1.0,
            }
        },
        0.0,
        0.0,
    )];

    let mut ball_list = Vec::<SetBallEvent>::new();
    ball_list.set_balls(BallType::Normal, 3);

    let goal_list = vec![SpawnGoalEvent::new(Vec2::new(200.0, 150.0), 20.0, 1)];

    StageInfo {
        blocks: field_block()
            .into_iter()
            .chain(block_list)
            .collect::<Vec<SpawnBlockEvent>>(),
        balls: ball_list,
        goal_pos: goal_list,
    }
}
