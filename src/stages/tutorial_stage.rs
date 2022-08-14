use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use super::structs::{
    ArrangeBallInfo, BlockInfo, BlockShapeInfo, GoalInfo, LauncherInfo, StageInfo, SwitchInfo,
};
use super::{field_blocks::field_block, structs::BallInfo};
use crate::components::ball::BallType;
use crate::components::block::{BlockSlidePath, RotateStrategy, SlideStrategy};
use crate::components::block_attach::fan::Fan;
use crate::components::block_attach::switch::{SwitchReceiver, SwitchType};
use crate::components::block_attach::BlockAttachment;
use crate::components::physics::material::PhysicMaterial;
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn tutorial_stage1() -> StageInfo {
    let material = PhysicMaterial::new(1.0, 1.0, 0.0);
    let block_list = vec![
        BlockInfo {
            pos: Vec2::new(0.0, 300.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(180.0, 90.0),
                rect_origin: Vec2::ZERO,
                rotate_strategy: RotateStrategy::NoRotate,
                slide_strategy: SlideStrategy::NoSlide,
            },
            material,
            default_angle: 1.0,
            default_pos_param: 0.0,
            block_attachment: vec![BlockAttachment::SwitchReceiver {
                receiver: SwitchReceiver {
                    switch_type: SwitchType::ChangeRotateStrategy {
                        before: RotateStrategy::NoRotate,
                        after: RotateStrategy::Auto(0.1),
                    },
                    target_id: 0,
                },
            }],
        },
        BlockInfo {
            pos: Vec2::new(200.0, 300.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(180.0, 90.0),
                rect_origin: Vec2::ZERO,
                rotate_strategy: RotateStrategy::Manual(0.05),
                slide_strategy: SlideStrategy::NoSlide,
            },
            material,
            default_angle: 0.0,
            default_pos_param: 0.0,
            block_attachment: vec![BlockAttachment::SwitchReceiver {
                receiver: SwitchReceiver {
                    switch_type: SwitchType::MoveBlock {
                        count: 0,
                        limit: 60,
                        func: |count: i32| Vec2::new(200.0, 300.0 - count as f32 * 2.0),
                    },
                    target_id: 0,
                },
            }],
        },
        BlockInfo {
            pos: Vec2::new(0.0, 0.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(200.0, 70.0),
                rect_origin: Vec2::ZERO,
                // rotate_strategy: RotateStrategy::Auto(0.01),
                rotate_strategy: RotateStrategy::NoRotate,
                // slide_strategy: SlideStrategy::Auto {
                //     speed: 0.05,
                //     path: BlockSlidePath::StandardLine {
                //         theta: 0.0,
                //         width: 50.0,
                //     },
                // },
                slide_strategy: SlideStrategy::NoSlide,
            },
            material,
            default_angle: 0.0,
            default_pos_param: 0.0,
            block_attachment: vec![BlockAttachment::Fan(Fan::new(true, 0, 0.0006))],
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
            pos: Vec2::new(0.0, -FIELD_HEIGHT / 2.0 + 30.0),
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
