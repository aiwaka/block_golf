use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::structs::{
    ArrangeBallInfo, BlockInfo, BlockShapeInfo, GoalInfo, LauncherInfo, StageInfo,
};
use super::{field_blocks::field_block, structs::BallInfo};
use crate::components::{ball::BallType, block::RotateStrategy};
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn debug_stage() -> StageInfo {
    let block_list = vec![
        BlockInfo {
            pos: Vec2::new(0.0, 0.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(50.0, 600.0),
            },
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(0.0, 0.0),
            block_shape_info: BlockShapeInfo::Ellipse {
                radii: Vec2::new(40.0, 60.0),
            },
            block_axis: Vec2::new(0.0, 10.0),
            rotate_strategy: RotateStrategy::infinite_manual(0.1),
            ..Default::default()
        },
    ];

    let launcher_info = LauncherInfo {
        pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, -FIELD_HEIGHT / 2.0 + 30.0),
        default_angle: 0.0,
        rotate_speed: 0.02,
        min_angle: 0.0,
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
        stage_title: "debug",
        time: 10 * 60,
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
