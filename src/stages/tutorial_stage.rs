use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::structs::{ArrangeBallInfo, BlockInfo, GoalInfo, LauncherInfo, StageInfo, SwitchInfo};
use super::{field_blocks::field_block, structs::BallInfo};
use crate::components::ball::BallType;
use crate::components::block_attach::switch::SwitchTile;
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn tutorial_stage1() -> StageInfo {
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

    let goal_list = vec![
        GoalInfo {
            pos: Vec2::new(FIELD_WIDTH / 2.0 - 30.0, -FIELD_HEIGHT / 2.0 + 30.0),
            radius: 40.0,
            score: 1,
        },
        GoalInfo {
            pos: Vec2::new(-FIELD_WIDTH / 2.0 + 20.0, FIELD_HEIGHT / 2.0 - 20.0),
            radius: 30.0,
            score: 2,
        },
    ];

    let switches = vec![
        SwitchInfo {
            target_id: 0,
            auto_reverse: Some(60),
            pos: Vec2::ZERO,
            ..Default::default()
        },
        SwitchInfo {
            default_active: true,
            target_id: 1,
            pos: Vec2::new(80.0, 0.0),
            ..Default::default()
        },
    ];

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
        switches,
    }
}
