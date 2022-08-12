use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::field_blocks::field_block;
use super::structs::{
    ArrangeBallInfo, BallInfo, BlockInfo, BlockShapeInfo, GoalInfo, LauncherInfo, StageInfo,
};
use crate::components::block::BlockSlidePath;
use crate::components::physics::material::PhysicMaterial;
use crate::components::{
    ball::BallType,
    block::{RotateStrategy, SlideStrategy},
};
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn jamming1() -> StageInfo {
    let material = PhysicMaterial::new(1.0, 1.0, 0.0);
    let mut block_list = Vec::<BlockInfo>::new();

    for i in 0..3 {
        for j in -2..=2 {
            block_list.push(BlockInfo {
                pos: Vec2::new(i as f32 * 40.0, j as f32 * 60.0),
                block_shape_info: BlockShapeInfo::Rect {
                    extents: Vec2::new(20.0, 40.0),
                    rect_origin: Vec2::ZERO,
                    rotate_strategy: RotateStrategy::Auto(0.1),
                    slide_strategy: SlideStrategy::NoSlide,
                },
                material,
                default_angle: (i * j) as f32,
                default_pos_param: 0.0,
            })
        }
    }
    let launcher_info = LauncherInfo {
        pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, 0.0),
        default_angle: 0.0,
        rotate_speed: 0.0,
        min_angle: 0.0,
        max_angle: 0.0,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 2);

    let goal_list = vec![GoalInfo {
        pos: Vec2::new(FIELD_WIDTH / 2.0 - 30.0, 0.0),
        radius: 30.0,
        score: 1,
    }];

    StageInfo {
        stage_title: "jamming1",
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
