use crate::components::block::{Block, BlockType, RectangleBlock, RotateStrategy, SpawnBlockEvent};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn test_set_block(mut event_writer: EventWriter<SpawnBlockEvent>) {
    let block_list = vec![
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::new(-230.0, 80.0),
                    extents: Vec2::new(90.0, 150.0),
                    rect_origin: Vec2::ZERO,
                    strategy: RotateStrategy::CannotRotate,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            2.0,
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::ZERO,
                    extents: Vec2::new(120.0, 80.0),
                    rect_origin: Vec2::new(30.0, 20.0),
                    strategy: RotateStrategy::Rotatable(0.015),
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            0.0,
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::new(200.0, 50.0),
                    extents: Vec2::new(120.0, 80.0),
                    rect_origin: Vec2::new(80.0, 0.0),
                    strategy: RotateStrategy::Always(0.02),
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            1.0,
        ),
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
                friction,
                restitution,
            } => {
                let block_shape = shapes::Rectangle {
                    extents: *extents,
                    origin: RectangleOrigin::CustomCenter(*rect_origin),
                };
                commands
                    .spawn_bundle(GeometryBuilder::build_as(
                        &block_shape.clone(),
                        DrawMode::Outlined {
                            fill_mode: FillMode::color(ev.color),
                            outline_mode: StrokeMode::new(Color::DARK_GRAY, 3.0),
                        },
                        Transform {
                            translation: Vec3::new(pos.x, pos.y, 12.0),
                            rotation: Quat::from_rotation_z(ev.default_angle),
                            ..Default::default()
                        },
                    ))
                    .insert(Block)
                    .insert(RectangleBlock {
                        rect: block_shape,
                        angle: ev.default_angle,
                        friction: *friction,
                        restitution: *restitution,
                    })
                    .insert(strategy.clone());
                commands.spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Circle {
                        radius: 10.0,
                        center: Vec2::new(0.0, 0.0),
                    },
                    DrawMode::Fill(FillMode::color(Color::RED)),
                    Transform {
                        translation: Vec3::new(pos.x, pos.y, 120.0),
                        ..Default::default()
                    },
                ))
            }
        };
    }
}

/// 回せるブロックと常に回るブロックを回す
fn rotate_block(
    key_in: Res<Input<KeyCode>>,
    mut block_query: Query<(&mut Transform, &mut RectangleBlock, &RotateStrategy), With<Block>>,
) {
    for (mut trans, mut rect, strategy) in block_query.iter_mut() {
        match strategy {
            RotateStrategy::CannotRotate => {}
            RotateStrategy::Rotatable(angle) => {
                if key_in.pressed(KeyCode::Left) {
                    rect.angle += angle;
                } else if key_in.pressed(KeyCode::Right) {
                    rect.angle -= angle;
                };
            }
            RotateStrategy::Always(angle) => {
                rect.angle += angle;
            }
        }
        trans.rotation = Quat::from_rotation_z(rect.angle);
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
