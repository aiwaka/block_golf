use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use super::field_blocks::field_block;
use super::structs::{
    ArrangeBallInfo, BallInfo, BlockInfo, BlockShapeInfo, GoalInfo, LauncherInfo, StageInfo,
};
use crate::components::{
    ball::BallType,
    block::{BlockSlidePath, RotateStrategy, SlideStrategy},
};
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn sample_stage() -> StageInfo {
    let block_list = vec![
        BlockInfo {
            pos: Vec2::new(-240.0, 70.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(90.0, 120.0),
            },
            slide_strategy: SlideStrategy::Manual {
                speed: 0.08,
                path: BlockSlidePath::StandardLine {
                    theta: PI,
                    width: 50.0,
                },
            },
            default_angle: 2.0,
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::ZERO,
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(120.0, 80.0),
            },
            block_axis: Vec2::new(30.0, 20.0),
            rotate_strategy: RotateStrategy::infinite_manual(0.025),
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(200.0, 50.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(120.0, 80.0),
            },
            block_axis: Vec2::new(80.0, 0.0),
            default_angle: 1.0,
            rotate_strategy: RotateStrategy::Auto(0.02),
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(300.0, -160.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(80.0, 30.0),
            },
            block_axis: Vec2::new(35.0, 0.0),
            rotate_strategy: RotateStrategy::infinite_manual(0.1),
            slide_strategy: SlideStrategy::AutoWrap {
                speed: 0.1,
                path: BlockSlidePath::StandardLine {
                    theta: FRAC_PI_2,
                    width: 40.0,
                },
            },
            default_angle: -FRAC_PI_2,
            default_pos_param: -1.0,
            ..Default::default()
        },
    ];

    let launcher_info = LauncherInfo {
        pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, -FIELD_HEIGHT / 2.0 + 30.0),
        default_angle: 0.0,
        rotate_speed: 0.02,
        min_angle: FRAC_PI_2 * -0.2,
        max_angle: FRAC_PI_2 * 1.2,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 10);

    let goal_list = vec![GoalInfo {
        pos: Vec2::new(350.0, 150.0),
        radius: 30.0,
        score: 1,
    }];

    StageInfo {
        stage_title: "sample",
        time: 60 * 60,
        launcher: launcher_info,
        blocks: field_block()
            .into_iter()
            .chain(block_list)
            .collect::<Vec<BlockInfo>>(),
        balls: ball_list,
        goal_pos: goal_list,
        switches: vec![],
        gravity: None,
    }
}
