use bevy::prelude::*;

use super::field_blocks::field_block;
use super::StageInfo;
use crate::components::block::{BlockType, RotateStrategy, SlideStrategy, SpawnBlockEvent};

pub fn debug_stage() -> StageInfo {
    let block_list = vec![SpawnBlockEvent::from_type(
        {
            BlockType::Rect {
                pos: Vec2::new(0.0, 0.0),
                extents: Vec2::new(50.0, 600.0),
                rect_origin: Vec2::ZERO,
                rotate_strategy: RotateStrategy::NoRotate,
                slide_strategy: SlideStrategy::NoSlide,
                weight: 1.0,
                friction: 0.0,
                restitution: 1.0,
            }
        },
        0.0,
        0.0,
    )];

    StageInfo {
        blocks: field_block()
            .into_iter()
            .chain(block_list)
            .collect::<Vec<SpawnBlockEvent>>(),
        ball_num: 1,
        hole_pos: Vec2::ZERO,
    }
}
