use std::f32::consts::{FRAC_PI_2, PI};

use crate::components::block::{
    Block, BlockSlidePath, BlockType, RectangleBlock, RotateStrategy, SlideStrategy,
    SpawnBlockEvent,
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::{
    block_slide_path::calc_orbit,
    field::{FIELD_HEIGHT, FIELD_WIDTH},
};

fn test_set_block(mut event_writer: EventWriter<SpawnBlockEvent>) {
    let field_block_list = vec![
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::new(FIELD_WIDTH / 2.0 + 30.0, 0.0),
                    extents: Vec2::new(60.0, FIELD_HEIGHT),
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
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::new(-FIELD_WIDTH / 2.0 - 30.0, 0.0),
                    extents: Vec2::new(60.0, FIELD_HEIGHT),
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
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::new(0.0, FIELD_HEIGHT / 2.0 + 30.0),
                    extents: Vec2::new(FIELD_WIDTH, 60.0),
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
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::new(0.0, -FIELD_HEIGHT / 2.0 - 30.0),
                    extents: Vec2::new(FIELD_WIDTH, 60.0),
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
        ),
    ];
    let block_list = vec![
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::new(-240.0, 70.0),
                    extents: Vec2::new(90.0, 120.0),
                    rect_origin: Vec2::ZERO,
                    rotate_strategy: RotateStrategy::NoRotate,
                    slide_strategy: SlideStrategy::Manual {
                        speed: 0.08,
                        path: BlockSlidePath::StandardLine {
                            theta: PI,
                            width: 50.0,
                        },
                    },
                    weight: 1.0,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            2.0,
            0.0,
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::ZERO,
                    extents: Vec2::new(120.0, 80.0),
                    rect_origin: Vec2::new(30.0, 20.0),
                    rotate_strategy: RotateStrategy::Manual(0.025),
                    slide_strategy: SlideStrategy::NoSlide,
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
                BlockType::Rect {
                    pos: Vec2::new(200.0, 50.0),
                    extents: Vec2::new(120.0, 80.0),
                    rect_origin: Vec2::new(80.0, 0.0),
                    rotate_strategy: RotateStrategy::Auto(0.02),
                    slide_strategy: SlideStrategy::NoSlide,
                    weight: 1.0,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            1.0,
            0.0,
        ),
        SpawnBlockEvent::from_type(
            {
                BlockType::Rect {
                    pos: Vec2::new(300.0, -120.0),
                    extents: Vec2::new(80.0, 30.0),
                    rect_origin: Vec2::new(35.0, 0.0),
                    rotate_strategy: RotateStrategy::Manual(0.1),
                    slide_strategy: SlideStrategy::AutoWrap {
                        speed: 0.1,
                        path: BlockSlidePath::StandardLine {
                            theta: FRAC_PI_2,
                            width: 40.0,
                        },
                    },
                    weight: 0.5,
                    friction: 0.0,
                    restitution: 1.0,
                }
            },
            0.0,
            -1.0,
        ),
    ];
    for e in field_block_list {
        event_writer.send(e)
    }
    for e in block_list {
        event_writer.send(e)
    }
}

/// キューに入っているブロックを追加する（開始時実行）
fn set_block(mut commands: Commands, mut event_listener: EventReader<SpawnBlockEvent>) {
    for ev in event_listener.iter() {
        match &ev.block_type {
            BlockType::Rect {
                pos,
                extents,
                rect_origin,
                rotate_strategy,
                slide_strategy,
                weight,
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
                        original_pos: *pos,
                        rect: block_shape,
                        angle: ev.default_angle,
                        pos_param: ev.default_pos_param,
                        weight: *weight,
                        friction: *friction,
                        restitution: *restitution,
                    })
                    .insert(rotate_strategy.clone())
                    .insert(slide_strategy.clone());
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
            RotateStrategy::NoRotate => {}
            RotateStrategy::Manual(angle) => {
                if key_in.pressed(KeyCode::Left) {
                    rect.angle += angle;
                } else if key_in.pressed(KeyCode::Right) {
                    rect.angle -= angle;
                };
            }
            RotateStrategy::Auto(angle) => {
                rect.angle += angle;
            }
        }
        trans.rotation = Quat::from_rotation_z(rect.angle);
    }
}

/// ブロックの移動処理を行う
fn slide_block(
    key_in: Res<Input<KeyCode>>,
    mut block_query: Query<(&mut Transform, &mut RectangleBlock, &SlideStrategy), With<Block>>,
) {
    for (mut trans, mut rect, strategy) in block_query.iter_mut() {
        let path = match strategy {
            SlideStrategy::NoSlide => &BlockSlidePath::NoPath,
            SlideStrategy::Manual { speed, path } => {
                if key_in.pressed(KeyCode::Left) {
                    rect.pos_param += speed;
                } else if key_in.pressed(KeyCode::Right) {
                    rect.pos_param -= speed;
                };
                if rect.pos_param > 1.0 {
                    rect.pos_param = 1.0;
                } else if rect.pos_param < -1.0 {
                    rect.pos_param = -1.0;
                }
                path
            }
            SlideStrategy::AutoWrap { speed, path } => {
                if key_in.pressed(KeyCode::Left) {
                    rect.pos_param += speed;
                } else if key_in.pressed(KeyCode::Right) {
                    rect.pos_param -= speed;
                };
                path
            }
            SlideStrategy::Auto { speed, path } => {
                rect.pos_param += speed;
                path
            }
        };
        let new_pos = calc_orbit(path, rect.pos_param) + rect.original_pos;
        trans.translation = Vec3::new(new_pos.x, new_pos.y, 12.0);
    }
}

pub struct BlockPlugin;
impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnBlockEvent>();
        app.add_startup_system(test_set_block);
        app.add_system(set_block);
        app.add_system(rotate_block);
        app.add_system(slide_block);
    }
}
