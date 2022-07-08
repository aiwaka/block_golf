use crate::components::block::{Block, BlockType, RotateStrategy, SpawnBlockEvent};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn test_set_block(mut event_writer: EventWriter<SpawnBlockEvent>) {
    let block_list = vec![
        SpawnBlockEvent::from_type({
            BlockType::Rect {
                pos: Vec2::ZERO,
                extents: Vec2::new(120.0, 80.0),
                rect_origin: Vec2::new(30.0, 0.0),
                strategy: RotateStrategy::CannotRotate,
            }
        }),
        SpawnBlockEvent::from_type({
            BlockType::Rect {
                pos: Vec2::ZERO,
                extents: Vec2::new(120.0, 80.0),
                rect_origin: Vec2::new(30.0, 0.0),
                strategy: RotateStrategy::Rotatable(0.015),
            }
        }),
        SpawnBlockEvent::from_type({
            BlockType::Rect {
                pos: Vec2::new(200.0, 50.0),
                extents: Vec2::new(120.0, 80.0),
                rect_origin: Vec2::new(80.0, 0.0),
                strategy: RotateStrategy::Always(0.02),
            }
        }),
    ];
    for e in block_list {
        event_writer.send(e)
    }
}

fn set_block(mut commands: Commands, mut event_listener: EventReader<SpawnBlockEvent>) {
    for ev in event_listener.iter() {
        match &ev.block_type {
            BlockType::Rect {
                pos,
                extents,
                rect_origin,
                strategy,
            } => {
                let block_shape = shapes::Rectangle {
                    extents: *extents,
                    origin: RectangleOrigin::CustomCenter(*rect_origin),
                };
                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &block_shape,
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(ev.color),
                            outline_mode: StrokeMode::new(Color::DARK_GRAY, 3.0),
                        },
                        Transform {
                            translation: Vec3::new(pos.x, pos.y, 12.0),
                            ..Default::default()
                        },
                    ))
                    .insert(Block)
                    .insert(strategy.clone());
            }
        };
    }
}

/// 回せるブロックと常に回るブロックを回す
fn rotate_block(
    key_in: Res<Input<KeyCode>>,
    mut block_query: Query<(&mut Transform, &RotateStrategy), With<Block>>,
) {
    for (mut transform, strategy) in block_query.iter_mut() {
        match strategy {
            RotateStrategy::CannotRotate => {}
            RotateStrategy::Rotatable(angle) => {
                if key_in.pressed(KeyCode::Right) || key_in.pressed(KeyCode::Left) {
                    if key_in.pressed(KeyCode::Right) {
                        transform.rotate(Quat::from_axis_angle(Vec3::Z, *angle));
                    } else {
                        transform.rotate(Quat::from_axis_angle(Vec3::Z, -*angle));
                    };
                }
            }
            RotateStrategy::Always(angle) => {
                transform.rotate(Quat::from_axis_angle(Vec3::Z, *angle));
            }
        }
    }
}

pub struct BlockPlugin;
impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBlockEvent>();
        app.add_startup_system(test_set_block);
        app.add_system(set_block);
        app.add_system(rotate_block);
    }
}
