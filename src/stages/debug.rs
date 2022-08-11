use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::structs::{ArrangeBallInfo, BlockInfo, GoalInfo, LauncherInfo, StageInfo};
use super::{field_blocks::field_block, structs::BallInfo};
use crate::components::{
    ball::BallType,
    block::{BlockType, RotateStrategy, SlideStrategy},
};
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn debug_stage() -> StageInfo {
    let block_list = vec![BlockInfo {
        block_type: BlockType::Rect {
            pos: Vec2::new(0.0, 0.0),
            extents: Vec2::new(50.0, 600.0),
            rect_origin: Vec2::ZERO,
            rotate_strategy: RotateStrategy::NoRotate,
            slide_strategy: SlideStrategy::NoSlide,
            weight: 1.0,
            friction: 0.0,
            restitution: 1.0,
        },
        default_angle: 0.0,
        default_pos_param: 0.0,
    }];

    let launcher_info = LauncherInfo {
        pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, -FIELD_HEIGHT / 2.0 + 30.0),
        rotate_speed: 0.02,
        min_angle: FRAC_PI_2,
        max_angle: FRAC_PI_2,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 10);

    let goal_list = vec![GoalInfo {
        pos: Vec2::new(200.0, 150.0),
        radius: 20.0,
        score: 1,
    }];

    StageInfo {
        time: 10 * 60,
        launcher: launcher_info,
        blocks: field_block()
            .into_iter()
            .chain(block_list)
            .collect::<Vec<BlockInfo>>(),
        balls: ball_list,
        goal_pos: goal_list,
    }
}
