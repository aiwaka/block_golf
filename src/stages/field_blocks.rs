use bevy::prelude::Vec2;

use crate::components::block::{BlockType, RotateStrategy, SlideStrategy, SpawnBlockEvent};
use crate::systems::field::{FIELD_HEIGHT, FIELD_WIDTH};

pub fn field_block() -> Vec<SpawnBlockEvent> {
    let field_block_list = vec![
        SpawnBlockEvent::from_type(
            {
                BlockType::Wall {
                    pos: Vec2::new(FIELD_WIDTH / 2.0 + 30.0, 0.0),
                    extents: Vec2::new(60.0, FIELD_HEIGHT),
                    weight: 1.0,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            0.0,
            0.0,
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Wall {
                    pos: Vec2::new(-FIELD_WIDTH / 2.0 - 30.0, 0.0),
                    extents: Vec2::new(60.0, FIELD_HEIGHT),
                    weight: 1.0,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            0.0,
            0.0,
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Wall {
                    pos: Vec2::new(0.0, FIELD_HEIGHT / 2.0 + 30.0),
                    extents: Vec2::new(FIELD_WIDTH, 60.0),
                    weight: 1.0,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            0.0,
            0.0,
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Wall {
                    pos: Vec2::new(0.0, -FIELD_HEIGHT / 2.0 - 30.0),
                    extents: Vec2::new(FIELD_WIDTH, 60.0),
                    weight: 1.0,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            0.0,
            0.0,
        ),
    ];
    field_block_list
}
