use bevy::prelude::Vec2;

use super::structs::{BlockInfo, BlockShapeInfo};
use crate::components::physics::material::PhysicMaterial;
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn field_block() -> Vec<BlockInfo> {
    let material = PhysicMaterial::new(1.0, 1.0, 0.0);
    vec![
        BlockInfo {
            pos: Vec2::new(FIELD_WIDTH / 2.0 + 30.0, 0.0),
            block_shape_info: BlockShapeInfo::Wall {
                extents: Vec2::new(60.0, FIELD_HEIGHT),
            },
            material,
            default_angle: 0.0,
            default_pos_param: 0.0,
            block_attachment: vec![],
        },
        BlockInfo {
            pos: Vec2::new(-FIELD_WIDTH / 2.0 - 30.0, 0.0),
            block_shape_info: BlockShapeInfo::Wall {
                extents: Vec2::new(60.0, FIELD_HEIGHT),
            },
            material,
            default_angle: 0.0,
            default_pos_param: 0.0,
            block_attachment: vec![],
        },
        BlockInfo {
            pos: Vec2::new(0.0, FIELD_HEIGHT / 2.0 + 30.0),
            block_shape_info: BlockShapeInfo::Wall {
                extents: Vec2::new(FIELD_WIDTH, 60.0),
            },
            material,
            default_angle: 0.0,
            default_pos_param: 0.0,
            block_attachment: vec![],
        },
        BlockInfo {
            pos: Vec2::new(0.0, -FIELD_HEIGHT / 2.0 - 30.0),
            block_shape_info: BlockShapeInfo::Wall {
                extents: Vec2::new(FIELD_WIDTH, 60.0),
            },
            material,
            default_angle: 0.0,
            default_pos_param: 0.0,
            block_attachment: vec![],
        },
    ]
}
