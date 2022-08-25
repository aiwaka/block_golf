use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;

use super::field_blocks::field_block;
use super::structs::{
    ArrangeBallInfo, BallInfo, BlockInfo, BlockShapeInfo, GoalInfo, LauncherInfo, StageInfo,
};
use crate::components::physics::force::Gravity;
use crate::components::physics::material::PhysicMaterial;
use crate::components::{ball::BallType, block::RotateStrategy};
use crate::systems::field::FIELD_WIDTH;

pub fn strange_gravity() -> StageInfo {
    let block_list = vec![];

    let launcher_info = LauncherInfo {
        pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, -250.0),
        default_angle: 0.0,
        rotate_speed: 0.0,
        min_angle: 0.0,
        max_angle: 0.0,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 1);
    ball_list.set_balls(BallType::Metal, 1);

    let goal_list = vec![GoalInfo {
        pos: Vec2::ZERO,
        radius: 20.0,
        score: 1,
    }];

    StageInfo {
        stage_title: "strange_gravity",
        time: 60 * 60,
        launcher: launcher_info,
        blocks: field_block()
            .into_iter()
            .chain(block_list)
            .collect::<Vec<BlockInfo>>(),
        balls: ball_list,
        goal_pos: goal_list,
        switches: vec![],
        gravity: Gravity::new_as_some(|pos: Vec2| -0.001 * pos),
    }
}

pub fn square_planet() -> StageInfo {
    let material = PhysicMaterial::new(0.8, 9.0, 1.0);
    let block_list = vec![
        BlockInfo {
            pos: Vec2::ZERO,
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(200.0, 200.0),
            },
            rotate_strategy: RotateStrategy::Auto(0.01),
            material,
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(50.0, 300.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(160.0, 40.0),
            },
            block_axis: Vec2::new(70.0, 0.0),
            rotate_strategy: RotateStrategy::infinite_manual(0.08),
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(50.0, -300.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(160.0, 40.0),
            },
            block_axis: Vec2::new(70.0, 0.0),
            rotate_strategy: RotateStrategy::infinite_manual(-0.08),
            ..Default::default()
        },
    ];

    let launcher_info = LauncherInfo {
        pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, 0.0),
        default_angle: 0.0,
        rotate_speed: 0.05,
        min_angle: -FRAC_PI_4,
        max_angle: FRAC_PI_4,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Metal, 2);
    ball_list.set_balls(BallType::Normal, 2);

    let goal_list = vec![
        GoalInfo {
            pos: Vec2::new(FIELD_WIDTH / 2.0 - 30.0, 0.0),
            radius: 30.0,
            score: 2,
        },
        GoalInfo {
            pos: Vec2::new(FIELD_WIDTH / 2.0 - 340.0, 0.0),
            radius: 30.0,
            score: 1,
        },
    ];

    StageInfo {
        stage_title: "square_planet",
        time: 60 * 60,
        launcher: launcher_info,
        blocks: field_block()
            .into_iter()
            .chain(block_list)
            .collect::<Vec<BlockInfo>>(),
        balls: ball_list,
        goal_pos: goal_list,
        switches: vec![],
        gravity: Gravity::new_as_some(|pos: Vec2| -0.0005 * pos),
    }
}
