use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use super::field_blocks::field_block;
use super::StageInfo;
use crate::components::{
    ball::{BallType, SetBall, SetBallEvent},
    block::{BlockSlidePath, BlockType, RotateStrategy, SlideStrategy, SpawnBlockEvent},
    goal::SpawnGoalEvent,
};

pub fn sample_stage() -> StageInfo {
    let block_list = vec![
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::new(-240.0, 70.0),
                    extents: Vec2::new(90.0, 120.0),
                    rect_origin: Vec2::ZERO,
                    rotate_strategy: RotateStrategy::NoRotate,
                    slide_strategy: SlideStrategy::Manual {
                        speed: 0.08,
                        path: BlockSlidePath::StandardLine {
                            theta: PI,
                            width: 50.0,
                        },
                    },
                    weight: 1.0,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            2.0,
            0.0,
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::ZERO,
                    extents: Vec2::new(120.0, 80.0),
                    rect_origin: Vec2::new(30.0, 20.0),
                    rotate_strategy: RotateStrategy::Manual(0.025),
                    slide_strategy: SlideStrategy::NoSlide,
                    weight: 1.0,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            0.0,
            0.0,
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::new(200.0, 50.0),
                    extents: Vec2::new(120.0, 80.0),
                    rect_origin: Vec2::new(80.0, 0.0),
                    rotate_strategy: RotateStrategy::Auto(0.02),
                    slide_strategy: SlideStrategy::NoSlide,
                    weight: 1.0,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            1.0,
            0.0,
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::new(300.0, -160.0),
                    extents: Vec2::new(80.0, 30.0),
                    rect_origin: Vec2::new(35.0, 0.0),
                    rotate_strategy: RotateStrategy::Manual(0.1),
                    slide_strategy: SlideStrategy::AutoWrap {
                        speed: 0.1,
                        path: BlockSlidePath::StandardLine {
                            theta: FRAC_PI_2,
                            width: 40.0,
                        },
                    },
                    weight: 0.5,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            -FRAC_PI_2,
            -1.0,
        ),
    ];

    let mut ball_list = Vec::<SetBallEvent>::new();
    ball_list.set_balls(BallType::Normal, 10);

    let goal_list = vec![SpawnGoalEvent::new(Vec2::new(350.0, 150.0), 30.0, 1)];

    StageInfo {
        blocks: field_block()
            .into_iter()
            .chain(block_list)
            .collect::<Vec<SpawnBlockEvent>>(),
        balls: ball_list,
        goal_pos: goal_list,
    }
}
