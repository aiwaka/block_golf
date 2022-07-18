use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use super::block_slide_path::calc_orbit;
use crate::components::block::{
    Block, BlockSlidePath, BlockType, RectangleBlock, RotateStrategy, SlideStrategy,
    SpawnBlockEvent,
};

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
        app.add_system(set_block);
        app.add_system(rotate_block);
        app.add_system(slide_block);
    }
}
