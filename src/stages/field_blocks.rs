use bevy::prelude::Vec2;

use super::structs::{BlockInfo, BlockShapeInfo};
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn field_block() -> Vec<BlockInfo> {
    vec![
        BlockInfo {
            pos: Vec2::new(FIELD_WIDTH / 2.0 + 30.0, 0.0),
            block_shape_info: BlockShapeInfo::Wall {
                extents: Vec2::new(60.0, FIELD_HEIGHT),
            },
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(-FIELD_WIDTH / 2.0 - 30.0, 0.0),
            block_shape_info: BlockShapeInfo::Wall {
                extents: Vec2::new(60.0, FIELD_HEIGHT),
            },
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(0.0, FIELD_HEIGHT / 2.0 + 30.0),
            block_shape_info: BlockShapeInfo::Wall {
                extents: Vec2::new(FIELD_WIDTH, 60.0),
            },
            ..Default::default()
        },
        BlockInfo {
            pos: Vec2::new(0.0, -FIELD_HEIGHT / 2.0 - 30.0),
            block_shape_info: BlockShapeInfo::Wall {
                extents: Vec2::new(FIELD_WIDTH, 60.0),
            },
            ..Default::default()
        },
    ]
}
