use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::Rectangle;

use crate::components::block::{BlockAxisPos, BlockSlideParam};
use crate::components::collision::RectangleCollision;
use crate::events::block::SpawnBlockEvent;
use crate::{
    components::{
        block::{
            BlockOriginalPos, BlockSlidePath, BlockTransformInfo, BlockType, RotateStrategy,
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
        let make_debug_bundle = |extents: Vec2| {
            GeometryBuilder::build_as(
                &Rectangle {
                    extents,
                    origin: RectangleOrigin::Center,
                },
                DrawMode::Outlined {
                    fill_mode: FillMode::color(Color::NONE),
                    outline_mode: StrokeMode::new(Color::ALICE_BLUE, 5.0),
                },
                Transform {
                    translation: Vec3::new(0.0, 0.0, 80.0),
                    ..Default::default()
                },
            )
        };
        let (shape_bundle, collision_ent) = match ev.block_type {
            BlockType::Wall { shape } => {
                let shape_bundle =
                    make_block_shape_bundle(&shape, color, ev.pos, z_offset, ev.default_angle);
                let collision_ent = commands
                    .spawn_bundle(make_debug_bundle(shape.extents))
                    .insert(RectangleCollision::new(shape.extents))
                    .id();
                (shape_bundle, collision_ent)
            }
            BlockType::Rect { shape } => {
                let shape_bundle =
                    make_block_shape_bundle(&shape, color, ev.pos, z_offset, ev.default_angle);
                let collision_ent = commands
                    .spawn()
                    // .spawn_bundle(make_debug_bundle(shape.extents))
                    .insert(RectangleCollision::new(shape.extents))
                    .id();
                (shape_bundle, collision_ent)
            }
            BlockType::Ellipse { shape } => {
                let shape_bundle =
                    make_block_shape_bundle(&shape, color, ev.pos, z_offset, ev.default_angle);
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
            .insert(ev.block_type.clone())
            .insert(BlockOriginalPos(ev.pos))
            .insert(BlockAxisPos(ev.block_axis))
            .insert(BlockTransformInfo::new(ev.default_angle, Vec2::ZERO))
            .insert(BlockSlideParam(ev.default_pos_param))
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
                    // TODO: ここもchildとして変更する
                    commands.entity(ent).insert(receiver.clone());
                }
                BlockAttachment::Fan(fan) => {
                    if let BlockType::Rect { shape } = ev.block_type {
                        spawn_fan(&mut commands, ent, &shape, fan);
                    }
                }
                BlockAttachment::Magnet(magnet) => {
                    if let BlockType::Rect { shape } = ev.block_type {
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
    mut block_query: Query<(&mut BlockTransformInfo, &Children)>,
    rot_str_query: Query<&RotateStrategy>,
) {
    for (mut block_trans, block_children) in block_query.iter_mut() {
        for &child in block_children.iter() {
            if let Ok(strategy) = rot_str_query.get(child) {
                match strategy {
                    RotateStrategy::NoRotate => {}
                    RotateStrategy::Manual(angle, min, max) => {
                        let current_angle = block_trans.angle;
                        block_trans.angle = (if key_in.pressed(KeyCode::Left) {
                            current_angle + angle
                        } else if key_in.pressed(KeyCode::Right) {
                            current_angle - angle
                        } else {
                            current_angle
                        })
                        .clamp(*min, *max);
                    }
                    RotateStrategy::Auto(angle) => {
                        block_trans.angle += angle;
                    }
                }
            }
        }
    }
}

/// ブロックの移動を計算する処理. ブロック移動のオフセットを計算する.
/// TODO: 現在パスは1つしかセットできないが, 配列に収めるようにして複数のパスを加算できるようにする.
fn slide_block(
    key_in: Res<Input<KeyCode>>,
    mut block_query: Query<(&mut BlockTransformInfo, &mut BlockSlideParam, &Children)>,
    sld_str_query: Query<&SlideStrategy>,
) {
    for (mut block_trans, mut slide_param, block_children) in block_query.iter_mut() {
        for &child in block_children.iter() {
            // ブロックの子であるスライド方法コンポーネントを取得
            if let Ok(strategy) = sld_str_query.get(child) {
                // ひとつ前のパラメータとして現在の値を保存
                let path = match strategy {
                    SlideStrategy::NoSlide => &BlockSlidePath::NoPath,
                    SlideStrategy::Manual { speed, path } => {
                        let current_param = slide_param.0;
                        // パラメータを[-1, 1]に収める
                        slide_param.0 = (if key_in.pressed(KeyCode::Left) {
                            current_param + speed
                        } else if key_in.pressed(KeyCode::Right) {
                            current_param - speed
                        } else {
                            current_param
                        })
                        .clamp(-1.0, 1.0);
                        path
                    }
                    SlideStrategy::AutoWrap { speed, path } => {
                        if key_in.pressed(KeyCode::Left) {
                            slide_param.0 += speed;
                        } else if key_in.pressed(KeyCode::Right) {
                            slide_param.0 -= speed;
                        };
                        path
                    }
                    SlideStrategy::Auto { speed, path } => {
                        slide_param.0 += speed;
                        path
                    }
                };
                block_trans.offset = path.calc_orbit(slide_param.0);
            }
        }
    }
}

/// ブロックの移動情報を実際の描画に反映する
fn reflect_block_transform(
    mut block_query: Query<(&mut Transform, &BlockTransformInfo, &BlockOriginalPos)>,
) {
    for (mut block_trans, block_trans_info, original_pos) in block_query.iter_mut() {
        let z_coord = block_trans.translation.z;
        block_trans.rotation = Quat::from_rotation_z(block_trans_info.angle);
        block_trans.translation = (block_trans_info.offset + original_pos.0).extend(z_coord);
    }
}

/// 矩形の当たり判定をブロックに追従させる.
fn update_rect_collision(
    block_q: Query<(
        &BlockTransformInfo,
        &BlockAxisPos,
        &BlockOriginalPos,
        &Children,
    )>,
    mut collision_q: Query<&mut RectangleCollision>,
) {
    for (block_trans, block_axis, block_orig_pos, block_children) in block_q.iter() {
        for &child in block_children.iter() {
            if let Ok(mut collision) = collision_q.get_mut(child) {
                collision.prev_pos = collision.pos;
                collision.prev_angle = collision.angle;
                collision.pos = block_orig_pos.0
                    + block_trans.offset
                    + Vec2::from_angle(block_trans.angle).rotate(block_axis.0);
                collision.angle = block_trans.angle;
            }
        }
    }
}

/// デバッグ用
#[derive(Component)]
struct DebugMarker;
fn debug_col(mut col_q: Query<(&mut Transform, &RectangleCollision), With<DebugMarker>>) {
    for (mut trans, col) in col_q.iter_mut() {
        trans.translation = col.pos.extend(80.0);
    }
}

pub struct BlockPlugin;
impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Game)
                .with_system(set_block)
                .after("spawn_stage_entities")
                .label("debug"),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(rotate_block)
                .label("block:rotate")
                .before("update_rect_collision"),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(slide_block)
                .label("block:slide")
                .before("update_rect_collision"),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(reflect_block_transform)
                .before("update_rect_collision")
                .after("block:rotate")
                .after("block:slide"),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game)
                .with_system(update_rect_collision)
                .label("update_rect_collision"),
        );
        // app.add_system_set(
        //     SystemSet::on_update(AppState::Game)
        //         .with_system(debug_collision)
        //         .after("update_rect_collision"),
        // );
    }
}
