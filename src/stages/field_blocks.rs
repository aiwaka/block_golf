use bevy::prelude::Vec2;

use super::structs::BlockInfo;
use crate::components::block::BlockType;
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn field_block() -> Vec<BlockInfo> {
    vec![
        BlockInfo {
            block_type: BlockType::Wall {
                pos: Vec2::new(FIELD_WIDTH / 2.0 + 30.0, 0.0),
                extents: Vec2::new(60.0, FIELD_HEIGHT),
                weight: 1.0,
                friction: 0.0,
                restitution: 1.0,
            },
            default_angle: 0.0,
            default_pos_param: 0.0,
        },
        BlockInfo {
            block_type: BlockType::Wall {
                pos: Vec2::new(-FIELD_WIDTH / 2.0 - 30.0, 0.0),
                extents: Vec2::new(60.0, FIELD_HEIGHT),
                weight: 1.0,
                friction: 0.0,
                restitution: 1.0,
            },
            default_angle: 0.0,
            default_pos_param: 0.0,
        },
        BlockInfo {
            block_type: BlockType::Wall {
                pos: Vec2::new(0.0, FIELD_HEIGHT / 2.0 + 30.0),
                extents: Vec2::new(FIELD_WIDTH, 60.0),
                weight: 1.0,
                friction: 0.0,
                restitution: 1.0,
            },
            default_angle: 0.0,
            default_pos_param: 0.0,
        },
        BlockInfo {
            block_type: BlockType::Wall {
                pos: Vec2::new(0.0, -FIELD_HEIGHT / 2.0 - 30.0),
                extents: Vec2::new(FIELD_WIDTH, 60.0),
                weight: 1.0,
                friction: 0.0,
                restitution: 1.0,
            },
            default_angle: 0.0,
            default_pos_param: 0.0,
        },
    ]
}
