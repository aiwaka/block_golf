use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::field_blocks::field_block;
use super::structs::{
    ArrangeBallInfo, BallInfo, BlockInfo, BlockShapeInfo, GoalInfo, LauncherInfo, StageInfo,
};
use crate::components::physics::material::PhysicMaterial;
use crate::components::{
    ball::BallType,
    block::{RotateStrategy, SlideStrategy},
};
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn aisle1() -> StageInfo {
    let material = PhysicMaterial::new(1.0, 1.0, 0.0);
    let orig_point = Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, -FIELD_HEIGHT / 2.0 + 30.0);
    const ROTATE_SPEED: f32 = 0.01;
    let block_list = vec![
        BlockInfo {
            pos: orig_point,
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(FIELD_WIDTH, FIELD_HEIGHT),
                rect_origin: Vec2::new(FIELD_WIDTH / 2.0 - 30.0, FIELD_HEIGHT / 2.0 + 80.0),
                rotate_strategy: RotateStrategy::Manual(ROTATE_SPEED),
                slide_strategy: SlideStrategy::NoSlide,
            },
            material,
            default_angle: 0.0,
            default_pos_param: 0.0,
        },
        BlockInfo {
            pos: orig_point,
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(FIELD_WIDTH, FIELD_HEIGHT),
                rect_origin: Vec2::new(FIELD_WIDTH / 2.0 - 30.0, -FIELD_HEIGHT / 2.0 - 80.0),
                rotate_strategy: RotateStrategy::Manual(ROTATE_SPEED),
                slide_strategy: SlideStrategy::NoSlide,
            },
            material,
            default_angle: 0.0,
            default_pos_param: 0.0,
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
    }
}
