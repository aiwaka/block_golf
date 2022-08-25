use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use itertools::Itertools;

use super::structs::{
    ArrangeBallInfo, BlockInfo, BlockShapeInfo, GoalInfo, LauncherInfo, StageInfo, SwitchInfo,
};
use super::{field_blocks::field_block, structs::BallInfo};
use crate::components::ball::BallType;
use crate::components::block::{BlockSlidePath, RotateStrategy, SlideStrategy};
use crate::components::block_attach::fan::Fan;
use crate::components::block_attach::magnet::Magnet;
use crate::components::block_attach::switch::{SwitchReceiver, SwitchType};
use crate::components::block_attach::BlockAttachment;
use crate::components::physics::force::Gravity;
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn tutorial1() -> StageInfo {
    let block_list = vec![
        BlockInfo {
            pos: Vec2::new(-100.0, 200.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(100.0, 80.0),
            },
            rotate_strategy: RotateStrategy::infinite_manual(0.06),
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(200.0, 200.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(100.0, 80.0),
            },
            slide_strategy: SlideStrategy::Manual {
                speed: 0.1,
                path: BlockSlidePath::StandardLine {
                    theta: 0.0,
                    width: 100.0,
                },
            },
            ..Default::default()
        },
    ];

    let launcher_info = LauncherInfo::default();

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 5);

    let goal_list = vec![GoalInfo {
        pos: Vec2::new(FIELD_WIDTH / 2.0 - 20.0, 0.0),
        radius: 40.0,
        score: 1,
    }];

    StageInfo {
        stage_title: "tutorial[1]",
        time: 30 * 60,
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

pub fn tutorial2() -> StageInfo {
    let block_list = vec![BlockInfo {
        pos: Vec2::ZERO,
        block_shape_info: BlockShapeInfo::Rect {
            extents: Vec2::new(200.0, 150.0),
        },
        ..Default::default()
    }];

    let launcher_info = LauncherInfo::default();

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 1);
    ball_list.set_balls(BallType::Metal, 1);

    let goal_list = vec![GoalInfo {
        pos: Vec2::new(FIELD_WIDTH / 2.0 - 20.0, 0.0),
        radius: 40.0,
        score: 1,
    }];

    StageInfo {
        stage_title: "tutorial[2]",
        time: 30 * 60,
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

pub fn fan_tutorial() -> StageInfo {
    let block_list = vec![
        BlockInfo {
            pos: Vec2::new(0.0, -300.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(200.0, 70.0),
            },
            block_attachment: vec![BlockAttachment::Fan(Fan::new(true, 0, 0.1))],
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(0.0, 250.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(200.0, 150.0),
            },
            ..Default::default()
        },
    ];

    let launcher_info = LauncherInfo {
        pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, FIELD_HEIGHT / 2.0 - 30.0),
        default_angle: 0.0,
        rotate_speed: 0.02,
        min_angle: -FRAC_PI_2,
        max_angle: 0.0,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 1);

    let goal_list = vec![GoalInfo {
        pos: Vec2::new(FIELD_WIDTH / 2.0 - 20.0, FIELD_HEIGHT / 2.0 - 20.0),
        radius: 40.0,
        score: 1,
    }];

    StageInfo {
        stage_title: "tutorial[fan]",
        time: 30 * 60,
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

pub fn magnet_tutorial() -> StageInfo {
    let block_list = vec![BlockInfo {
        pos: Vec2::new(0.0, 250.0),
        block_shape_info: BlockShapeInfo::Rect {
            extents: Vec2::new(200.0, 150.0),
        },
        block_attachment: vec![BlockAttachment::Magnet(Magnet::new(true, 1, 14.0))],
        ..Default::default()
    }];

    let launcher_info = LauncherInfo {
        pos: Vec2::new(-FIELD_WIDTH / 2.0 + 30.0, FIELD_HEIGHT / 2.0 - 30.0),
        default_angle: 0.0,
        rotate_speed: 0.02,
        min_angle: -FRAC_PI_2,
        max_angle: 0.0,
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Metal, 1);
    ball_list.set_balls(BallType::Normal, 1);

    let goal_list = vec![
        GoalInfo {
            pos: Vec2::new(FIELD_WIDTH / 2.0 - 20.0, FIELD_HEIGHT / 2.0 - 20.0),
            radius: 40.0,
            score: 1,
        },
        GoalInfo {
            pos: Vec2::new(FIELD_WIDTH / 2.0 - 20.0, -FIELD_HEIGHT / 2.0 + 50.0),
            radius: 40.0,
            score: 1,
        },
    ];

    StageInfo {
        stage_title: "tutorial[magnet]",
        time: 30 * 60,
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

pub fn switch_tutorial() -> StageInfo {
    const ROTATE_FUNC: fn(i32) -> f32 = |param: i32| FRAC_PI_2 / 30.0 * param as f32;
    let block_shape_info = BlockShapeInfo::Rect {
        extents: Vec2::new(120.0, 30.0),
    };
    let block_list = vec![
        BlockInfo {
            pos: Vec2::new(340.0, -160.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(120.0, 30.0),
            },
            block_axis: Vec2::new(60.0, 0.0),
            block_attachment: vec![BlockAttachment::SwitchReceiver {
                receiver: SwitchReceiver {
                    switch_type: SwitchType::RotateBlock {
                        range: (-30..=30).rev().collect_vec(),
                        func: ROTATE_FUNC,
                    },
                    target_id: 0,
                },
            }],
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(400.0, -40.0),
            block_shape_info: block_shape_info.clone(),
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(400.0, -160.0),
            block_shape_info: block_shape_info.clone(),
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(0.0, 170.0),
            block_shape_info: block_shape_info.clone(),
            block_attachment: vec![BlockAttachment::SwitchReceiver {
                receiver: SwitchReceiver {
                    switch_type: SwitchType::MoveBlock {
                        range: (0..40).collect_vec(),
                        func: |count: i32| Vec2::new(-count as f32 * 3.0, 0.0),
                    },
                    target_id: 1,
                },
            }],
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(60.0, 220.0),
            block_shape_info: block_shape_info.clone(),
            default_angle: FRAC_PI_2,
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(-60.0, 220.0),
            block_shape_info,
            default_angle: FRAC_PI_2,
            ..Default::default()
        },
    ];

    let launcher_info = LauncherInfo {
        pos: Vec2::new(0.0, -100.0),
        rotate_speed: 0.08,
        min_angle: 0.0,
        max_angle: FRAC_PI_2,
        ..Default::default()
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Normal, 3);

    let goal_list = vec![
        GoalInfo {
            pos: Vec2::new(450.0, -100.0),
            radius: 40.0,
            score: 1,
        },
        GoalInfo {
            pos: Vec2::new(0.0, 220.0),
            radius: 30.0,
            score: 2,
        },
    ];

    let switches = vec![
        SwitchInfo {
            target_id: 0,
            auto_reverse: None,
            pos: Vec2::new(40.0, -100.0),
            ..Default::default()
        },
        SwitchInfo {
            target_id: 1,
            auto_reverse: Some(90),
            pos: Vec2::new(100.0, -100.0),
            ..Default::default()
        },
    ];

    StageInfo {
        stage_title: "tutorial[switch]",
        time: 30 * 60,
        launcher: launcher_info,
        blocks: field_block()
            .into_iter()
            .chain(block_list)
            .collect::<Vec<BlockInfo>>(),
        balls: ball_list,
        goal_pos: goal_list,
        switches,
        // gravity: Some(Gravity::simple_gravity()),
        gravity: None,
    }
}

pub fn gravity_tutorial() -> StageInfo {
    let block_shape_info = BlockShapeInfo::Rect {
        extents: Vec2::new(FIELD_WIDTH / 3.0, 150.0),
    };
    let block_list = vec![
        BlockInfo {
            pos: Vec2::new(0.0, -100.0),
            block_shape_info: BlockShapeInfo::Rect {
                extents: Vec2::new(FIELD_WIDTH / 3.0, 150.0),
            },
            slide_strategy: SlideStrategy::Manual {
                speed: 0.05,
                path: BlockSlidePath::StandardLine {
                    theta: FRAC_PI_2,
                    width: 120.0,
                },
            },
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(-FIELD_WIDTH / 3.0, 0.0),
            block_shape_info: block_shape_info.clone(),
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(FIELD_WIDTH / 3.0, 0.0),
            block_shape_info,
            ..Default::default()
        },
    ];

    let launcher_info = LauncherInfo {
        pos: Vec2::new(-400.0, 200.0),
        ..Default::default()
    };

    let mut ball_list = Vec::<BallInfo>::new();
    ball_list.set_balls(BallType::Metal, 1);

    let goal_list = vec![
        GoalInfo {
            pos: Vec2::new(FIELD_WIDTH / 2.0 - 20.0, 90.0),
            radius: 40.0,
            score: 1,
        },
        GoalInfo {
            pos: Vec2::new(80.0, 170.0),
            radius: 40.0,
            score: 3,
        },
    ];

    StageInfo {
        stage_title: "tutorial[gravity]",
        time: 30 * 60,
        launcher: launcher_info,
        blocks: field_block()
            .into_iter()
            .chain(block_list)
            .collect::<Vec<BlockInfo>>(),
        balls: ball_list,
        goal_pos: goal_list,
        switches: vec![],
        gravity: Gravity::new_as_some(|_: Vec2| Vec2::Y * (-1.5)),
    }
}
