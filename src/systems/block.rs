use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;

use crate::components::collision::RectangleCollision;
use crate::events::block::SpawnBlockEvent;
use crate::{
    components::{
        block::{
            Block, BlockOriginalPos, BlockSlidePath, BlockTransform, BlockType, RotateStrategy,
            SlideStrategy,
        },
        block_attach::BlockAttachment,
    },
    AppState,
};

use super::block_attach::fan::spawn_fan;
use super::block_attach::magnet::spawn_magnet;

fn make_block_shape_bundle(
    shape: &impl Geometry,
    color: Color,
    pos: Vec2,
    z_offset: f32,
    angle: f32,
) -> ShapeBundle {
    GeometryBuilder::build_as(
        shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(color),
            outline_mode: StrokeMode::new(Color::DARK_GRAY, 3.0),
        },
        Transform {
            translation: pos.extend(10.5 + z_offset),
            rotation: Quat::from_rotation_z(angle),
            ..Default::default()
        },
    )
}

/// キューに入っているブロックを追加する（開始時実行）
fn set_block(mut commands: Commands, mut event_listener: EventReader<SpawnBlockEvent>) {
    for (idx, ev) in event_listener.iter().enumerate() {
        // ブロックが重なったときに変な表示にならないようにz座標に微妙な差をつける
        let z_offset = idx as f32 / 1000.0;
        let color = Color::from(&ev.block_type);
        let (shape_bundle, collision_ent) = match ev.block_type {
            BlockType::Wall { shape } => {
                let shape_bundle = make_block_shape_bundle(
                    &shape,
                    color,
                    ev.pos,
                    10.5 + z_offset,
                    ev.default_angle,
                );
                let collision_ent = commands
                    .spawn()
                    .insert(RectangleCollision::new(shape.extents))
                    .id();
                (shape_bundle, collision_ent)
            }
            BlockType::Rect { shape } => {
                let shape_bundle = make_block_shape_bundle(
                    &shape,
                    color,
                    ev.pos,
                    12.0 + z_offset,
                    ev.default_angle,
                );
                let collision_ent = commands
                    .spawn()
                    .insert(RectangleCollision::new(shape.extents))
                    .id();
                (shape_bundle, collision_ent)
            }
            BlockType::Ellipse { shape } => {
                let shape_bundle = make_block_shape_bundle(
                    &shape,
                    color,
                    ev.pos,
                    12.0 + z_offset,
                    ev.default_angle,
                );
                // TODO: あとで楕円のものに直す
                let collision_ent = commands
                    .spawn()
                    .insert(RectangleCollision::new(shape.radii))
                    .id();
                (shape_bundle, collision_ent)
            }
        };
        let ent = commands
            .spawn_bundle(shape_bundle)
            .insert(Block)
            .insert(ev.block_type.clone())
            .insert(BlockOriginalPos(ev.pos))
            .insert(BlockTransform::new(ev.default_angle, ev.default_pos_param))
            .insert(ev.material)
            .id();
        // 子コンポーネントとして回転・移動方法, 当たり判定を追加
        let child_rot_str_ent = commands.spawn().insert(ev.rotate_strategy.clone()).id();
        let child_sld_str_ent = commands.spawn().insert(ev.slide_strategy.clone()).id();
        commands
            .entity(ent)
            .push_children(&[child_rot_str_ent, child_sld_str_ent, collision_ent]);
        // ブロックにくっつけるものを追加.
        for com in ev.block_attachment.iter() {
            match com {
                BlockAttachment::SwitchReceiver { receiver } => {
                    commands.entity(ent).insert(receiver.clone());
                }
                BlockAttachment::Fan(fan) => {
                    if let BlockType::Rect { shape } = ev.block_type {
                        commands.entity(ent).insert(fan.clone());
                        spawn_fan(&mut commands, ent, &shape, fan);
                    }
                }
                BlockAttachment::Magnet(magnet) => {
                    if let BlockType::Rect { shape } = ev.block_type {
                        commands.entity(ent).insert(magnet.clone());
                        spawn_magnet(&mut commands, ent, &shape, magnet);
                    }
                }
            }
        }
        // commands.spawn_bundle(GeometryBuilder::build_as(
        //     &shapes::Circle {
        //         radius: 10.0,
        //         center: Vec2::new(0.0, 0.0),
        //     },
        //     DrawMode::Fill(FillMode::color(Color::RED)),
        //     Transform {
        //         translation: ev.pos.extend(80.0),
        //         ..Default::default()
        //     },
        // ));
    }
}

/// 回せるブロックと常に回るブロックを回す
fn rotate_block(
    key_in: Res<Input<KeyCode>>,
    mut block_query: Query<(&mut Transform, &mut BlockTransform, &Children), With<Block>>,
    rot_str_query: Query<&RotateStrategy>,
) {
    for (mut trans, mut block_trans, block_children) in block_query.iter_mut() {
        for &child in block_children.iter() {
            if let Ok(strategy) = rot_str_query.get(child) {
                // ひとつ前のパラメータとして現在の値を保存
                // block_trans.prev_angle = block_trans.angle;
                match strategy {
                    RotateStrategy::NoRotate => {}
                    RotateStrategy::Manual(angle, min, max) => {
                        if key_in.pressed(KeyCode::Left) {
                            block_trans.angle += angle;
                        } else if key_in.pressed(KeyCode::Right) {
                            block_trans.angle -= angle;
                        };
                        if block_trans.angle > *max {
                            block_trans.angle = *max;
                        }
                        if block_trans.angle < *min {
                            block_trans.angle = *min;
                        }
                    }
                    RotateStrategy::Auto(angle) => {
                        block_trans.angle += angle;
                    }
                }
                trans.rotation = Quat::from_rotation_z(block_trans.angle);
            }
        }
    }
}

/// ブロックの移動処理を行う. オフセットを加えるのもここで行う.
fn slide_block(
    key_in: Res<Input<KeyCode>>,
    mut block_query: Query<
        (
            &mut Transform,
            &mut BlockTransform,
            &BlockOriginalPos,
            &Children,
        ),
        With<Block>,
    >,
    sld_str_query: Query<&SlideStrategy>,
) {
    for (mut trans, mut block_trans, original_pos, block_children) in block_query.iter_mut() {
        for &child in block_children.iter() {
            if let Ok(strategy) = sld_str_query.get(child) {
                // ひとつ前のパラメータとして現在の値を保存
                // block_trans.prev_param = block_trans.pos_param;
                // block_trans.prev_offset = block_trans.offset;
                let path = match strategy {
                    SlideStrategy::NoSlide => &BlockSlidePath::NoPath,
                    SlideStrategy::Manual { speed, path } => {
                        if key_in.pressed(KeyCode::Left) {
                            block_trans.pos_param += speed;
                        } else if key_in.pressed(KeyCode::Right) {
                            block_trans.pos_param -= speed;
                        };
                        if block_trans.pos_param > 1.0 {
                            block_trans.pos_param = 1.0;
                        } else if block_trans.pos_param < -1.0 {
                            block_trans.pos_param = -1.0;
                        }
                        path
                    }
                    SlideStrategy::AutoWrap { speed, path } => {
                        if key_in.pressed(KeyCode::Left) {
                            block_trans.pos_param += speed;
                        } else if key_in.pressed(KeyCode::Right) {
                            block_trans.pos_param -= speed;
                        };
                        path
                    }
                    SlideStrategy::Auto { speed, path } => {
                        block_trans.pos_param += speed;
                        path
                    }
                };
                let new_pos =
                    path.calc_orbit(block_trans.pos_param) + block_trans.offset + original_pos.0;
                let z_coord = trans.translation.z;
                trans.translation = new_pos.extend(z_coord);
            }
        }
    }
}

// fn temp(q: Query<(&RectangleBlock, &SlideStrategy, &RotateStrategy), With<Block>>) {
//     for (rec, sl, ro) in q.iter() {
//         if let SlideStrategy::NoSlide = sl {
//             if let RotateStrategy::Manual(angle) = ro {
//                 info!("current ang vel: {}", rec.angle_diff());
//             }
//         }
//     }
// }

pub struct BlockPlugin;
impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(set_block)
                .after("spawn_stage_entities"),
        );
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(rotate_block));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(slide_block));
        // app.add_system_set(SystemSet::on_update(AppState::Game).with_system(temp));
    }
}
