use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::structs::{
    ArrangeBallInfo, BlockInfo, BlockShapeInfo, GoalInfo, LauncherInfo, StageInfo,
};
use super::{field_blocks::field_block, structs::BallInfo};
use crate::components::physics::material::PhysicMaterial;
use crate::components::{
    ball::BallType,
    block::{RotateStrategy, SlideStrategy},
};
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn tutorial_stage1() -> StageInfo {
    let material = PhysicMaterial::new(1.0, 1.0, 0.0);
    let block_list = vec![];

    let launcher_info = LauncherInfo {
        pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, -FIELD_HEIGHT / 2.0 + 30.0),
        default_angle: 0.0,
        rotate_speed: 0.02,
        min_angle: 0.0,
        max_angle: FRAC_PI_2,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 5);

    let goal_list = vec![GoalInfo {
        pos: Vec2::new(FIELD_WIDTH / 2.0 - 30.0, -FIELD_HEIGHT / 2.0 + 30.0),
        radius: 40.0,
        score: 1,
    }];

    StageInfo {
        stage_title: "tutorial1",
        time: 30 * 60,
        launcher: launcher_info,
        blocks: field_block()
            .into_iter()
            .chain(block_list)
            .collect::<Vec<BlockInfo>>(),
        balls: ball_list,
        goal_pos: goal_list,
    }
}
