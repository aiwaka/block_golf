use std::f32::consts::FRAC_PI_4;

use bevy::prelude::*;
use itertools::Itertools;

use super::field_blocks::field_block;
use super::structs::{
    ArrangeBallInfo, BallInfo, BlockInfo, BlockShapeInfo, GoalInfo, LauncherInfo, StageInfo,
    SwitchInfo,
};
use crate::components::block_attach::switch::{SwitchReceiver, SwitchType};
use crate::components::block_attach::BlockAttachment;
use crate::components::physics::material::PhysicMaterial;
use crate::components::{
    ball::BallType,
    block::{RotateStrategy, SlideStrategy},
};
use crate::systems::field::FIELD_WIDTH;

pub fn jamming1() -> StageInfo {
    let material = PhysicMaterial::new(1.0, 1.0, 0.0);
    let mut block_list = Vec::<BlockInfo>::new();

    // ブロックが退避する動き
    const ESCAPE_MOVE_0: fn(i32) -> Vec2 = |count: i32| Vec2::new(0.0, -count as f32 * 6.0);
    const ESCAPE_MOVE_1: fn(i32) -> Vec2 = |count: i32| Vec2::new(0.0, count as f32 * 6.0);

    for i in 0..4i32 {
        for j in -2..2i32 {
            let pos_x = (i / 2) as f32 * 220.0 + (i % 2) as f32 * 60.0 - 50.0;
            let pos_y = j as f32 * 120.0 + 60.0;
            let block_attachment = vec![BlockAttachment::SwitchReceiver {
                receiver: SwitchReceiver {
                    switch_type: SwitchType::MoveBlock {
                        range: (0..60).collect_vec(),
                        func: if j < 0 { ESCAPE_MOVE_0 } else { ESCAPE_MOVE_1 },
                    },
                    target_id: (i / 2) as u32,
                },
            }];

            // 回転の向きを決める
            // iが奇数なら前半が1.0, 偶数なら前半が-1.0
            // jが-2, -1なら後半が-1.0, 0, 1なら1.0になる.
            let rotate_sgn = -1.0 * ((i % 2) * 2 - 1).signum() as f32 * (j as f32 + 0.5).signum();

            block_list.push(BlockInfo {
                pos: Vec2::new(pos_x, pos_y),
                block_shape_info: BlockShapeInfo::Rect {
                    extents: Vec2::new(30.0, 100.0),
                    rect_origin: Vec2::ZERO,
                    rotate_strategy: RotateStrategy::Auto(0.08 * rotate_sgn),
                    slide_strategy: SlideStrategy::NoSlide,
                },
                material,
                default_angle: (i * j) as f32,
                default_pos_param: 0.0,
                block_attachment,
            })
        }
    }
    let launcher_info = LauncherInfo {
        pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, 0.0),
        default_angle: 0.0,
        rotate_speed: 0.05,
        min_angle: -FRAC_PI_4,
        max_angle: FRAC_PI_4,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 3);

    let goal_list = vec![
        GoalInfo {
            pos: Vec2::new(-120.0, 0.0),
            radius: 30.0,
            score: 1,
        },
        GoalInfo {
            pos: Vec2::new(300.0, 0.0),
            radius: 30.0,
            score: 3,
        },
    ];

    let switches = vec![
        SwitchInfo {
            target_id: 0,
            auto_reverse: None,
            pos: Vec2::new(-400.0, 0.0),
            ..Default::default()
        },
        SwitchInfo {
            target_id: 1,
            auto_reverse: Some(90),
            pos: Vec2::new(-320.0, 0.0),
            ..Default::default()
        },
    ];

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
        switches,
        gravity: None,
    }
}
