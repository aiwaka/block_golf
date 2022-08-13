use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::*,
    shapes::{Ellipse, Rectangle},
};

use crate::{
    components::{
        block::{BlockType, RotateStrategy, SlideStrategy},
        block_attach::BlockAttachment,
        physics::material::PhysicMaterial,
    },
    stages::structs::{BlockInfo, BlockShapeInfo},
};

use super::ToSpawnEvent;

/// タイプと色を指定
pub struct SpawnBlockEvent {
    pub pos: Vec2,
    pub block_type: BlockType,
    pub material: PhysicMaterial,
    pub default_angle: f32,
    pub default_pos_param: f32,
    pub rotate_strategy: RotateStrategy,
    pub slide_strategy: SlideStrategy,
    pub block_attachment: Vec<BlockAttachment>,
}
impl ToSpawnEvent for BlockInfo {
    type E = SpawnBlockEvent;
    fn to_spawn_event(&self) -> Self::E {
        match &self.block_shape_info {
            BlockShapeInfo::Wall { extents } => {
                let block_type = BlockType::Wall {
                    shape: Rectangle {
                        extents: *extents,
                        origin: RectangleOrigin::CustomCenter(Vec2::ZERO),
                    },
                };
                SpawnBlockEvent {
                    pos: self.pos,
                    block_type,
                    material: self.material,
                    default_angle: self.default_angle,
                    default_pos_param: self.default_pos_param,
                    rotate_strategy: RotateStrategy::NoRotate,
                    slide_strategy: SlideStrategy::NoSlide,
                    block_attachment: self.block_attachment.clone(),
                }
            }
            BlockShapeInfo::Rect {
                extents,
                rect_origin,
                rotate_strategy,
                slide_strategy,
            } => {
                let block_type = BlockType::Rect {
                    shape: Rectangle {
                        extents: *extents,
                        origin: RectangleOrigin::CustomCenter(*rect_origin),
                    },
                };
                SpawnBlockEvent {
                    pos: self.pos,
                    block_type,
                    material: self.material,
                    default_angle: self.default_angle,
                    default_pos_param: self.default_pos_param,
                    rotate_strategy: rotate_strategy.clone(),
                    slide_strategy: slide_strategy.clone(),
                    block_attachment: self.block_attachment.clone(),
                }
            }
            BlockShapeInfo::Ellipse {
                radii,
                center,
                rotate_strategy,
                slide_strategy,
            } => {
                let block_type = BlockType::Ellipse {
                    shape: Ellipse {
                        radii: *radii,
                        center: *center,
                    },
                };
                SpawnBlockEvent {
                    pos: self.pos,
                    block_type,
                    material: self.material,
                    default_angle: self.default_angle,
                    default_pos_param: self.default_pos_param,
                    rotate_strategy: rotate_strategy.clone(),
                    slide_strategy: slide_strategy.clone(),
                    block_attachment: self.block_attachment.clone(),
                }
            }
        }
    }
}
