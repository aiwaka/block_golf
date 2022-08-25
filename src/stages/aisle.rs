use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use super::field_blocks::field_block;
use super::structs::{
    ArrangeBallInfo, BallInfo, BlockInfo, BlockShapeInfo, GoalInfo, LauncherInfo, StageInfo,
};
use crate::components::block::BlockSlidePath;
use crate::components::{
    ball::BallType,
    block::{RotateStrategy, SlideStrategy},
};
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn aisle0() -> StageInfo {
    let orig_point = Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, -FIELD_HEIGHT / 2.0 + 30.0);
    const ROTATE_SPEED: f32 = 0.01;
    let block_list = vec![
        BlockInfo {
            pos: Vec2::new(-130.0, -60.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(FIELD_WIDTH / 2.0, FIELD_HEIGHT / 2.0),
            },
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(130.0, 60.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(FIELD_WIDTH / 2.0, FIELD_HEIGHT / 2.0),
            },
            ..Default::default()
        },
    ];

    let launcher_info = LauncherInfo {
        pos: orig_point,
        default_angle: 0.0,
        rotate_speed: ROTATE_SPEED,
        min_angle: 0.0,
        max_angle: FRAC_PI_2,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 2);

    let goal_list = vec![GoalInfo {
        pos: Vec2::from_angle(PI).rotate(orig_point),
        radius: 50.0,
        ..Default::default()
    }];

    StageInfo {
        stage_title: "aisle0",
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

pub fn aisle1() -> StageInfo {
    let orig_point = Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, -FIELD_HEIGHT / 2.0 + 30.0);
    const ROTATE_SPEED: f32 = 0.01;
    let block_list = vec![
        BlockInfo {
            pos: orig_point,
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(FIELD_WIDTH, FIELD_HEIGHT),
            },
            block_axis: Vec2::new(FIELD_WIDTH / 2.0 - 30.0, FIELD_HEIGHT / 2.0 + 80.0),
            rotate_strategy: RotateStrategy::Manual(ROTATE_SPEED, 0.0, FRAC_PI_2),
            ..Default::default()
        },
        BlockInfo {
            pos: orig_point,
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(FIELD_WIDTH, FIELD_HEIGHT),
            },
            block_axis: Vec2::new(FIELD_WIDTH / 2.0 - 30.0, -FIELD_HEIGHT / 2.0 - 80.0),
            rotate_strategy: RotateStrategy::Manual(ROTATE_SPEED, 0.0, FRAC_PI_2),
            ..Default::default()
        },
    ];

    let launcher_info = LauncherInfo {
        pos: orig_point,
        default_angle: 0.0,
        rotate_speed: ROTATE_SPEED,
        min_angle: 0.0,
        max_angle: FRAC_PI_2,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 2);

    let goal_list = vec![GoalInfo {
        pos: orig_point + Vec2::Y * FIELD_HEIGHT * 0.8,
        radius: 30.0,
        score: 1,
    }];

    StageInfo {
        stage_title: "aisle1",
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

pub fn aisle2() -> StageInfo {
    const ROTATE_SPEED: f32 = 0.01;
    let block_list = vec![
        BlockInfo {
            pos: Vec2::new(400.0, -100.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(FIELD_WIDTH, FIELD_HEIGHT),
            },
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(-FIELD_WIDTH / 2.0 - 30.0, FIELD_HEIGHT / 2.0 - 60.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(160.0, 100.0),
            },
            slide_strategy: SlideStrategy::Manual {
                speed: 0.07,
                path: BlockSlidePath::StandardLine {
                    theta: 0.0,
                    width: -160.0,
                },
            },
            ..Default::default()
        },
    ];

    let launcher_info = LauncherInfo {
        pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, -FIELD_HEIGHT / 2.0 + 30.0),
        default_angle: FRAC_PI_2,
        rotate_speed: ROTATE_SPEED,
        min_angle: 0.0,
        max_angle: FRAC_PI_2,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 3);

    let goal_list = vec![GoalInfo {
        pos: Vec2::new(FIELD_WIDTH / 2.0 - 20.0, FIELD_HEIGHT / 2.0 - 40.0),
        radius: 50.0,
        score: 1,
    }];

    StageInfo {
        stage_title: "aisle2",
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
