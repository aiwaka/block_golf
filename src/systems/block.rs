use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

use crate::{
    components::block::{
        Block, BlockOriginalPos, BlockSlidePath, BlockTransform, BlockType, RotateStrategy,
        SlideStrategy, SpawnBlockEvent,
    },
    AppState,
};

/// キューに入っているブロックを追加する（開始時実行）
fn set_block(mut commands: Commands, mut event_listener: EventReader<SpawnBlockEvent>) {
    for ev in event_listener.iter() {
        let color = Color::from(&ev.block_type);
        let shape_bundle = match ev.block_type {
            BlockType::Wall { shape } => GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(color),
                    outline_mode: StrokeMode::new(Color::DARK_GRAY, 3.0),
                },
                Transform {
                    translation: ev.pos.extend(12.0),
                    rotation: Quat::from_rotation_z(ev.default_angle),
                    ..Default::default()
                },
            ),
            BlockType::Rect { shape } => GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(color),
                    outline_mode: StrokeMode::new(Color::DARK_GRAY, 3.0),
                },
                Transform {
                    translation: ev.pos.extend(12.0),
                    rotation: Quat::from_rotation_z(ev.default_angle),
                    ..Default::default()
                },
            ),
            BlockType::Ellipse { shape } => GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(color),
                    outline_mode: StrokeMode::new(Color::DARK_GRAY, 3.0),
                },
                Transform {
                    translation: ev.pos.extend(12.0),
                    rotation: Quat::from_rotation_z(ev.default_angle),
                    ..Default::default()
                },
            ),
        };
        commands
            .spawn_bundle(shape_bundle)
            .insert(Block)
            .insert(ev.block_type.clone())
            .insert(BlockOriginalPos(ev.pos))
            .insert(BlockTransform::new(ev.default_angle, ev.default_pos_param))
            .insert(ev.material)
            .insert(ev.rotate_strategy.clone())
            .insert(ev.slide_strategy.clone());
        commands.spawn_bundle(GeometryBuilder::build_as(
            &shapes::Circle {
                radius: 10.0,
                center: Vec2::new(0.0, 0.0),
            },
            DrawMode::Fill(FillMode::color(Color::RED)),
            Transform {
                translation: ev.pos.extend(80.0),
                ..Default::default()
            },
        ));
    }
}

/// 回せるブロックと常に回るブロックを回す
fn rotate_block(
    key_in: Res<Input<KeyCode>>,
    mut block_query: Query<(&mut Transform, &mut BlockTransform, &RotateStrategy), With<Block>>,
) {
    for (mut trans, mut block_trans, strategy) in block_query.iter_mut() {
        // ひとつ前のパラメータとして現在の値を保存
        block_trans.prev_angle = block_trans.angle;
        match strategy {
            RotateStrategy::NoRotate => {}
            RotateStrategy::Manual(angle) => {
                if key_in.pressed(KeyCode::Left) {
                    block_trans.angle += angle;
                } else if key_in.pressed(KeyCode::Right) {
                    block_trans.angle -= angle;
                };
            }
            RotateStrategy::Auto(angle) => {
                block_trans.angle += angle;
            }
        }
        trans.rotation = Quat::from_rotation_z(block_trans.angle);
    }
}

/// ブロックの移動処理を行う
fn slide_block(
    key_in: Res<Input<KeyCode>>,
    mut block_query: Query<
        (
            &mut Transform,
            &mut BlockTransform,
            &SlideStrategy,
            &BlockOriginalPos,
        ),
        With<Block>,
    >,
) {
    for (mut trans, mut block_trans, strategy, original_pos) in block_query.iter_mut() {
        // ひとつ前のパラメータとして現在の値を保存
        block_trans.prev_param = block_trans.pos_param;
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
        let new_pos = path.calc_orbit(block_trans.pos_param) + original_pos.0;
        trans.translation = Vec3::new(new_pos.x, new_pos.y, 12.0);
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
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(set_block));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(rotate_block));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(slide_block));
        // app.add_system_set(SystemSet::on_update(AppState::Game).with_system(temp));
    }
}
