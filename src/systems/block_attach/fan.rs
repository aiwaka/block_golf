use bevy::prelude::*;
use bevy_prototype_lyon::{
    prelude::{DrawMode, FillMode, GeometryBuilder},
    shapes::{Circle, Rectangle, RectangleOrigin},
};

use crate::{
    components::{
        ball::Ball,
        block::{BlockTransformInfo, BlockType},
        block_attach::fan::{Fan, WindVisualEffect},
        block_attach::utils::EdgeDirection,
        physics::{force::Force, material::Volume, position::Position, velocity::Velocity},
        timer::CountDownTimer,
    },
    systems::utils::calc_edge_points_of_rectangle,
    AppState,
};

/// ブロック出現時に送風機のポリゴンを描画するときに使う関数
pub fn spawn_fan(commands: &mut Commands, block_ent: Entity, rect: &Rectangle, fan: &Fan) {
    let (fan_extents, fan_pos) = match fan.direction {
        EdgeDirection::Up => (
            rect.extents.project_onto(Vec2::X) + Vec2::Y * 10.0,
            rect.extents.project_onto(Vec2::Y) / 2.0,
        ),
        EdgeDirection::Down => (
            rect.extents.project_onto(Vec2::X) + Vec2::Y * 10.0,
            -rect.extents.project_onto(Vec2::Y) / 2.0,
        ),
        EdgeDirection::Left => (
            rect.extents.project_onto(Vec2::Y) + Vec2::X * 10.0,
            -rect.extents.project_onto(Vec2::X) / 2.0,
        ),
        EdgeDirection::Right => (
            rect.extents.project_onto(Vec2::Y) + Vec2::X * 10.0,
            rect.extents.project_onto(Vec2::X) / 2.0,
        ),
    };
    let fan_shape_bundle = GeometryBuilder::build_as(
        &Rectangle {
            extents: fan_extents,
            origin: RectangleOrigin::CustomCenter(Vec2::ZERO),
        },
        DrawMode::Fill(FillMode::color(Color::BLUE)),
        Transform {
            translation: fan_pos.extend(16.0),
            ..Default::default()
        },
    );
    let child_ent = commands
        .spawn()
        .insert_bundle(fan_shape_bundle)
        .insert(fan.clone())
        .id();
    commands.entity(block_ent).push_children(&[child_ent]);
}

/// 風エフェクトのためにここでだけ使うタイマーコンポーネント
#[derive(Component, Debug)]
struct WindVfxDuration(Timer);
fn set_wind_vfx_duration(mut commands: Commands) {
    commands
        .spawn()
        .insert(WindVfxDuration(Timer::from_seconds(0.12, true)));
}
// 風エフェクトを出す
fn spawn_wind_visual_effect(
    mut commands: Commands,
    block_query: Query<(&BlockTransformInfo, &GlobalTransform, &BlockType, &Children)>,
    fan_query: Query<&Fan>,
    time: Res<Time>,
    mut timer_query: Query<&mut WindVfxDuration>,
) {
    let mut wind_vfx_timer = timer_query.single_mut();
    if wind_vfx_timer.0.tick(time.delta()).just_finished() {
        let effect_shape = Circle {
            radius: 5.0,
            center: Vec2::ZERO,
        };
        let effect_draw_mode = DrawMode::Fill(FillMode::color(Color::WHITE));
        for (block_trans, block_glb_trans, block_type, block_children) in block_query.iter() {
            for &child in block_children.iter() {
                if let Ok(fan) = fan_query.get(child) {
                    if fan.active {
                        if let BlockType::Rect { shape } = block_type {
                            let angle = block_trans.angle;
                            let (_, _, block_glb_translation) =
                                block_glb_trans.to_scale_rotation_translation();
                            // まずファンの両端点を計算する
                            let [p1, p2] = calc_edge_points_of_rectangle(
                                &fan.direction,
                                block_glb_translation.truncate(),
                                angle,
                                shape.extents,
                            );
                            // 経過時刻を用いてエフェクトを出す.
                            // [0,1]を取るパラメータで内分して位置を計算
                            let param =
                                (time.seconds_since_startup() as f32 * 60.0).sin() / 2.0 + 0.5;
                            let spawn_pos = p1.lerp(p2, param);
                            let effect_vel = (p1 - p2).perp().normalize() * 15.0;
                            commands
                                .spawn_bundle(GeometryBuilder::build_as(
                                    &effect_shape,
                                    effect_draw_mode,
                                    Transform {
                                        translation: spawn_pos.extend(50.0),
                                        ..Default::default()
                                    },
                                ))
                                .insert(WindVisualEffect)
                                .insert(Velocity(effect_vel))
                                .insert(Position(spawn_pos))
                                .insert(CountDownTimer::new(60));
                        }
                    }
                }
            }
        }
    }
}

/// 風エフェクトを更新する. タイマーにより削除は自動的に行われる
fn update_wind_visual_effect(
    mut query: Query<(&mut Transform, &Position), With<WindVisualEffect>>,
) {
    for (mut trans, pos) in query.iter_mut() {
        trans.translation = pos.0.extend(50.0);
    }
}

/// 動いている送風機とボールの間に障害物がなければ力を加える
fn generate_wind(
    block_query: Query<(&BlockTransformInfo, &GlobalTransform, &BlockType, &Children)>,
    fan_query: Query<&Fan>,
    mut ball_query: Query<(&Ball, &Position, &Volume, &mut Force)>,
) {
    for (block_trans, block_glb_trans, block_type, block_children) in block_query.iter() {
        for &child in block_children.iter() {
            if let Ok(fan) = fan_query.get(child) {
                if fan.active {
                    if let BlockType::Rect { shape } = block_type {
                        let angle = block_trans.angle;
                        // まずファンの両端点を計算する
                        let (_, _, block_glb_translation) =
                            block_glb_trans.to_scale_rotation_translation();
                        let [p1, p2] = calc_edge_points_of_rectangle(
                            &fan.direction,
                            block_glb_translation.truncate(),
                            angle,
                            shape.extents,
                        );

                        for (_, ball_pos, volume, mut force) in ball_query.iter_mut() {
                            let ball_pos = ball_pos.0;
                            if (p2 - p1).dot(ball_pos - p1) > 0.0
                                && (p1 - p2).dot(ball_pos - p2) > 0.0
                                && (ball_pos - p1).perp_dot(p2 - p1) > 0.0
                            {
                                let dir_unit = (p1 - p2).perp().normalize();
                                // TODO: また, 障害物を挟んだ場合風が届かないようにしたい.
                                force.0 += dir_unit * fan.pressure * volume.0;
                                // force.0 = force.0.clamp_length_max(15.0);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub(super) struct FanPlugin;
impl Plugin for FanPlugin {
    fn build(&self, app: &mut App) {
        // app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(temp));
        app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(set_wind_vfx_duration));
        app.add_system_set(SystemSet::on_update(AppState::Game).with_system(generate_wind));
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(spawn_wind_visual_effect),
        );
        app.add_system_set(
            SystemSet::on_update(AppState::Game).with_system(update_wind_visual_effect),
        );
    }
}
